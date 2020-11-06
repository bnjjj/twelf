use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse::Parse, punctuated::Punctuated, DeriveInput, Error, Expr, Ident, Lit, Token};
#[derive(Default, Debug)]
struct ConfigAttrs {
    prefix: Option<String>,
}

impl Parse for ConfigAttrs {
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

        let mut config_attrs = ConfigAttrs::default();
        for (assign_expr, right_expr) in assign_exprs {
            let name = assign_expr
                .path
                .get_ident()
                .ok_or_else(|| {
                    Error::new_spanned(assign_expr.clone(), "config attribute must have a name")
                })?
                .to_string();

            if name == "prefix" {
                if let Expr::Lit(lit_expr) = *right_expr {
                    if let Lit::Str(lit_expr) = &lit_expr.lit {
                        config_attrs.prefix = Some(lit_expr.value());
                        break;
                    } else {
                        return Err(Error::new_spanned(
                            lit_expr,
                            "prefix value must be str literal",
                        ));
                    }
                } else {
                    return Err(Error::new_spanned(
                        right_expr,
                        "prefix value must be literal",
                    ));
                }
            }
        }

        Ok(config_attrs)
    }
}

#[proc_macro_attribute]
pub fn config(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let strukt = syn::parse_macro_input!(item as DeriveInput);
    let struct_name = strukt.ident.clone();
    let attrs = syn::parse_macro_input!(attrs as ConfigAttrs);

    let fields = if let syn::Data::Struct(fields) = &strukt.data {
        fields.fields.iter().map(|field| {
            let ty = field.ty.clone();
            let ident = field.ident.clone();
            let vis = field.vis.clone();
            let colon = field.colon_token;
            quote!(#vis #ident #colon Option<#ty>,)
        })
    } else {
        return TokenStream::from(
            Error::new_spanned(strukt, "must be a struct").to_compile_error(),
        );
    };
    let struct_vis = strukt.vis.clone();
    let struct_gen = strukt.generics.clone();
    let struct_attrs = strukt.attrs.clone();
    let opt_struct_name = Ident::new(
        format!("Opt{}", struct_name.to_string()).as_str(),
        Span::call_site(),
    );

    let opt_struct = quote! {
        #(#struct_attrs)*
        #struct_vis struct #opt_struct_name #struct_gen {
            #(#fields)*
        }
    };

    // strukt.data
    let body = match attrs.prefix {
        Some(prefix) => {
            quote!(config::reexports::envy::prefixed(#prefix).from_env::<#struct_name>())
        }
        None => quote!(config::reexports::envy::from_env::<#struct_name>()),
    };
    let code = quote! {
        #[derive(config::reexports::serde::Deserialize)]
        #strukt

        impl #struct_name {
            pub fn new() -> config::reexports::envy::Result<Self> {
                #body
            }

            pub fn with_priorities(priorities: &[config::Priority]) -> Result<Self, config::Error> {
                use std::iter::FromIterator;
                let mut res: std::collections::HashMap<String, config::reexports::serde_json::Value> = std::collections::HashMap::new();
                for prio in priorities {
                    let extension = Self::parse(prio)?;
                    // println!("extension --- {:?}", extension);
                    res.extend(
                        extension
                            .as_object()
                            .ok_or_else(|| config::Error::InvalidFormat)?
                            .to_owned()
                            .into_iter().filter(|(_k, v)| !v.is_null()),
                    );
                }

                config::reexports::serde_json::from_value(config::reexports::serde_json::Value::Object(config::reexports::serde_json::Map::from_iter(res.into_iter()))).map_err(config::Error::from)
            }

            // TODO: use this for parsing in the builder loop priorities, maybe use a trait ? or add with priorities to loop over here. maybe it's better
            fn parse(priority: &config::Priority) -> Result<config::reexports::serde_json::Value, config::Error>
            {
                 #[derive(config::reexports::serde::Deserialize, config::reexports::serde::Serialize)]
                #opt_struct

                let res = match priority {
                    config::Priority::Env(prefix) => match prefix {
                        Some(prefix) => {
                            let tmp_cfg: #opt_struct_name = config::reexports::envy::prefixed(prefix).from_env()?;
                            serde_json::to_value(tmp_cfg)

                        },
                        None => {
                            let tmp_cfg: #opt_struct_name = config::reexports::envy::from_env()?;
                            serde_json::to_value(tmp_cfg)
                        },
                    }?,
                    config::Priority::Json(filepath) => config::reexports::serde_json::from_reader(std::fs::File::open(filepath)?)?,
                    config::Priority::Toml(filepath) => config::reexports::toml::from_str(&std::fs::read_to_string(filepath)?)?,
                    config::Priority::Yaml(_) => todo!(),
                };

                Ok(res)
            }
        }
    };

    TokenStream::from(code)
}
