use std::collections::HashMap;

use heck::KebabCase;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse::Parse, punctuated::Punctuated, DeriveInput, Error, Expr, Ident, Lit, Meta, Token,
};
#[derive(Default, Debug)]
struct AssignAttrs;

impl Parse for AssignAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;

        let assign_exprs = attrs.into_iter().filter_map(|attr| {
            if let Expr::Assign(assign_expr) = attr {
                if let Expr::Path(path) = *assign_expr.left {
                    return Some((path, assign_expr.right));
                }
            }

            None
        });

        for (assign_expr, _right_expr) in assign_exprs {
            let name = assign_expr
                .path
                .get_ident()
                .ok_or_else(|| {
                    Error::new_spanned(assign_expr.clone(), "config attribute must have a name")
                })?
                .to_string();

            if name == "default"
                || name == "serialize_with"
                || name == "deserialize_with"
                || name == "with"
            {
                return Ok(Self {});
            }
        }

        Err(Error::new(Span::call_site(), "not found"))
    }
}

#[proc_macro_attribute]
pub fn config(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let strukt = syn::parse_macro_input!(item as DeriveInput);
    let struct_name = strukt.ident.clone();
    // let attrs = syn::parse_macro_input!(attrs as ConfigAttrs);
    let mut fields_name = vec![];
    let mut fields_doc: HashMap<String, Option<String>> = HashMap::new();
    let fields = if let syn::Data::Struct(fields) = &strukt.data {
        fields.fields.iter().map(|field| {
            let ty = field.ty.clone();
            let ident = field.ident.clone();
            let vis = field.vis.clone();
            let colon = field.colon_token;

            if let Some(ident) = &ident {
                fields_name.push(ident.to_string());
                fields_doc.insert(ident.to_string(), None);
            }

            let attrs =
                field
                    .attrs
                    .clone()
                    .into_iter()
                    .filter(|attr| match attr.path.get_ident() {
                        Some(attr_ident) if attr_ident == "serde" => {
                            attr.parse_args::<AssignAttrs>().is_err()
                        }
                        Some(attr_ident) if attr_ident == "doc" => {
                            let doc = match attr.parse_meta() {
                                Ok(Meta::NameValue(ref nv)) if nv.path.is_ident("doc") => {
                                    if let Lit::Str(lit_str) = &nv.lit {
                                        Some(lit_str.value())
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            };
                            if let Some(ident) = &ident {
                                *fields_doc.get_mut(&ident.to_string()).unwrap() = doc;
                            }
                            true
                        }
                        _ => true,
                    });
            let res = quote! {
                #(#attrs)*
                #vis #ident #colon Option<#ty>,
            };

            res
        })
    } else {
        return TokenStream::from(
            Error::new_spanned(strukt, "must be a struct").to_compile_error(),
        );
    };
    let struct_vis = strukt.vis.clone();
    let struct_gen = strukt.generics.clone();
    let struct_where = strukt.generics.where_clause.clone();
    let struct_attrs = strukt.attrs.clone();
    let opt_struct_name = Ident::new(
        format!("Opt{}", struct_name.to_string()).as_str(),
        Span::call_site(),
    );

    let opt_struct = quote! {
        #(#struct_attrs)*
        #struct_vis struct #opt_struct_name #struct_gen #struct_where {
            #(#fields)*
        }
    };

    let docs = fields_doc.values().map(|doc| match doc {
        Some(doc) => doc.trim().to_string(),
        None => String::new(),
    });
    let field_names_clap = fields_name
        .clone()
        .into_iter()
        .map(|ref field_name| field_name.to_kebab_case());

    let code = quote! {
        #[derive(::twelf::reexports::serde::Deserialize)]
        #strukt

        impl #struct_gen #struct_name #struct_gen #struct_where {
            pub fn with_layers(layers: &[::twelf::Layer]) -> Result<Self, ::twelf::Error> {
                use std::iter::FromIterator;
                let mut res: std::collections::HashMap<String, ::twelf::reexports::serde_json::Value> = std::collections::HashMap::new();
                for layer in layers {
                    let extension = Self::parse(layer)?;
                    res.extend(
                        extension
                            .as_object()
                            .ok_or_else(|| ::twelf::Error::InvalidFormat)?
                            .to_owned()
                            .into_iter().filter(|(_k, v)| !v.is_null()),
                    );
                }

                ::twelf::reexports::serde_json::from_value(::twelf::reexports::serde_json::Value::Object(::twelf::reexports::serde_json::Map::from_iter(res.into_iter()))).map_err(|e| ::twelf::Error::Deserialize(e.to_string()))
            }

            pub fn clap_args() -> Vec<::twelf::reexports::clap::Arg<'static, 'static>> {
                vec![#(
                   ::twelf::reexports::clap::Arg::with_name(#fields_name).long(#field_names_clap).help(#docs).takes_value(true).global(true)
                ),*]
            }

            fn parse(priority: &::twelf::Layer) -> Result<::twelf::reexports::serde_json::Value, ::twelf::Error>
            {
                 #[derive(::twelf::reexports::serde::Deserialize, ::twelf::reexports::serde::Serialize)]
                #opt_struct

                let res = match priority {
                    ::twelf::Layer::Env(prefix) => match prefix {
                        Some(prefix) => {
                            let tmp_cfg: #opt_struct_name #struct_gen = ::twelf::reexports::envy::prefixed(prefix).from_env()?;
                            ::twelf::reexports::serde_json::to_value(tmp_cfg)
                        },
                        None => {
                            let tmp_cfg: #opt_struct_name #struct_gen = ::twelf::reexports::envy::from_env()?;
                            ::twelf::reexports::serde_json::to_value(tmp_cfg)
                        },
                    }?,
                    ::twelf::Layer::Json(filepath) => ::twelf::reexports::serde_json::from_reader(std::fs::File::open(filepath)?)?,
                    ::twelf::Layer::Toml(filepath) => ::twelf::reexports::toml::from_str(&std::fs::read_to_string(filepath)?)?,
                    ::twelf::Layer::Toml(filepath) => ::twelf::reexports::toml::from_str(&std::fs::read_to_string(filepath)?)?,
                    ::twelf::Layer::Yaml(filepath) => ::twelf::reexports::serde_yaml::from_str(&std::fs::read_to_string(filepath)?)?,
                    ::twelf::Layer::Ini(filepath) => {
                       let tmp_cfg: #opt_struct_name #struct_gen = ::twelf::reexports::serde_ini::from_str(&std::fs::read_to_string(filepath)?)?;
                       ::twelf::reexports::serde_json::to_value(tmp_cfg)?
                    },
                    ::twelf::Layer::Clap(matches) => {
                        let mut map: std::collections::HashMap<String, String> = std::collections::HashMap::new();

                        #(
                            if let Some(vmatch) = matches.value_of(#fields_name) {
                                map.insert(String::from(#fields_name), vmatch.to_string());
                            }
                        )*

                        let tmp_cfg: #opt_struct_name #struct_gen = ::twelf::reexports::envy::from_iter(map.into_iter())?;
                        ::twelf::reexports::serde_json::to_value(tmp_cfg)?
                    },
                };

                Ok(res)
            }
        }
    };

    TokenStream::from(code)
}
