mod attr;

use std::collections::HashMap;

#[cfg(feature = "clap")]
use heck::ToKebabCase;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
#[cfg(any(feature = "env", feature = "clap", feature = "ini"))]
use syn::Generics;
use syn::{DeriveInput, Error, Ident, Lit, Meta, Type};

use crate::attr::AssignAttrs;

/// proc macro to declare a configuration structure
#[proc_macro_attribute]
pub fn config(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let strukt = syn::parse_macro_input!(item as DeriveInput);
    let struct_name = strukt.ident.clone();
    // let attrs = syn::parse_macro_input!(attrs as ConfigAttrs);
    let mut fields_name = vec![];
    let mut fields_doc: HashMap<String, Option<String>> = HashMap::new();
    let mut fields_is_boolean = vec![];

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
            // Detect boolean types for clap
            match &ty {
                Type::Path(pat) => {
                    if let Some(type_name) = pat.path.get_ident() {
                        // let ident_name = type_name.to_string();
                        if type_name == "bool" {
                            fields_is_boolean.push(true);
                        } else {
                            fields_is_boolean.push(false);
                        }
                    } else {
                        fields_is_boolean.push(false);
                    }
                }
                _ => {
                    fields_is_boolean.push(false);
                }
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
                        Some(attr_ident) if attr_ident == "clap" => false,
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
    let struct_attrs =
        strukt.attrs.clone().into_iter().filter(
            |attr| matches!(attr.path.get_ident(), Some(attr_ident) if attr_ident == "serde"),
        );
    let opt_struct_name = Ident::new(format!("Opt{}", struct_name).as_str(), Span::call_site());

    let opt_struct = quote! {
        #(#struct_attrs)*
        #struct_vis struct #opt_struct_name #struct_gen #struct_where {
            #(#fields)*
        }
    };

    #[cfg(feature = "clap")]
    let docs = fields_doc.values().map(|doc| match doc {
        Some(doc) => doc.trim().to_string(),
        None => String::new(),
    });

    let json_branch = build_json_branch();
    let toml_branch = build_toml_branch();
    let yaml_branch = build_yaml_branch();
    let dhall_branch = build_dhall_branch();
    let default_trait_branch = build_default_trait_branch();
    let custom_fn_branch = build_custom_fn_branch();

    #[cfg(not(feature = "ini"))]
    let ini_branch = quote! {};
    #[cfg(feature = "ini")]
    let ini_branch = build_ini_branch(&opt_struct_name, &struct_gen);

    #[cfg(not(feature = "env"))]
    let env_branch = quote! {};
    #[cfg(feature = "env")]
    let env_branch = build_env_branch(&opt_struct_name, &struct_gen);

    #[cfg(not(feature = "clap"))]
    let (clap_branch, clap_method) = (quote! {}, quote! {});
    #[cfg(feature = "clap")]
    let (clap_branch, clap_method) = build_clap_branch(
        &opt_struct_name,
        &struct_gen,
        &fields_name,
        &fields_is_boolean,
        docs,
    );

    #[cfg(all(not(feature = "default_trait"), not(feature = "custom_fn")))]
    let derive_serialize = quote! {};
    #[cfg(any(feature = "default_trait", feature = "custom_fn"))]
    let derive_serialize = quote! { #[derive(::twelf::reexports::serde::Serialize)] };

    let code = quote! {
        #derive_serialize
        #[derive(::twelf::reexports::serde::Deserialize)]
        #[serde(crate = "::twelf::reexports::serde")]
        #strukt

        impl #struct_gen #struct_name #struct_gen #struct_where {
            pub fn with_layers(layers: &[::twelf::Layer]) -> Result<Self, ::twelf::Error> {
                use std::iter::FromIterator;
                let mut res: std::collections::HashMap<String, ::twelf::reexports::serde_json::Value> = std::collections::HashMap::new();
                for layer in layers {
                    let (extension,defaulted) = Self::parse_twelf(layer)?;
                    let extension: std::collections::HashMap<_,_> = extension
                        .as_object()
                        .ok_or_else(|| ::twelf::Error::InvalidFormat)?
                        .to_owned()
                        .into_iter().filter(|(k, v)| (!defaulted.contains_key(k) || !defaulted[k] || !res.contains_key(k)) && !v.is_null())
                        .collect(); // must collect, as filter uses res

                    res.extend(extension);
                }

                ::twelf::reexports::log::debug!(target: "twelf", "configuration:");
                for (key, val) in &res {
                    ::twelf::reexports::log::debug!(target: "twelf", "{}={}", key, val);
                }

                ::twelf::reexports::serde_json::from_value(::twelf::reexports::serde_json::Value::Object(::twelf::reexports::serde_json::Map::from_iter(res.into_iter()))).map_err(|e| ::twelf::Error::Deserialize(format!("config error: {}", e.to_string())))
            }
            #clap_method

            fn parse_twelf(priority: &::twelf::Layer) -> Result<(::twelf::reexports::serde_json::Value,std::collections::HashMap<String,bool>), ::twelf::Error>
            {
                #[derive(::twelf::reexports::serde::Deserialize, ::twelf::reexports::serde::Serialize)]
                #[serde(crate = "::twelf::reexports::serde")]
                #opt_struct

                let (res,defaulted) = match priority {
                    #env_branch
                    #json_branch
                    #toml_branch
                    #yaml_branch
                    #dhall_branch
                    #ini_branch
                    #clap_branch
                    #default_trait_branch
                    #custom_fn_branch
                    other => unimplemented!("{:?}", other)
                };

                Ok((res,defaulted.unwrap_or(std::collections::HashMap::new())))
            }
        }
    };

    TokenStream::from(code)
}

#[cfg(feature = "clap")]
fn build_clap_branch(
    opt_struct_name: &Ident,
    struct_gen: &Generics,
    fields_name: &[String],
    fields_is_boolean: &[bool],
    docs: impl Iterator<Item = String>,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let field_names_clap = fields_name
        .iter()
        .map(|field_name| field_name.to_kebab_case());

    let field_names_clap_cloned = field_names_clap.clone();
    let clap_branch = quote! { ::twelf::Layer::Clap(matches) => {
        let mut map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        let mut defaulted: std::collections::HashMap<String, bool> = std::collections::HashMap::new();

        #(
            let field = String::from(#fields_name);

            let mut insert_into_map = |vals: ::twelf::reexports::clap::parser::RawValues, is_default: bool| {
                let mut key = field.clone();
                defaulted.insert(key.clone(), is_default);

                for val in vals.into_iter() {
                    // hacky way of formatting everything to a string:
                    let s = val.to_string_lossy().into_owned();
                    let s = s.strip_prefix("\"").unwrap_or(&s);
                    let s = s.strip_suffix("\"").unwrap_or(&s);

                    if let Some(existing_val) = map.get_mut(&key) {
                        *existing_val = existing_val.clone() + "," + s;
                    } else {
                        map.insert(key.clone(), s.to_string());
                    }
                }
            };

            if let Some(vals) = matches.try_get_raw(#fields_name).unwrap_or(None) {
                let is_default = matches.value_source(#fields_name).unwrap() == ::twelf::reexports::clap::parser::ValueSource::DefaultValue;
                insert_into_map(vals, is_default);
            } else if let Some(vals) = matches.try_get_raw(#field_names_clap_cloned).unwrap_or(None) {
                let is_default = matches.value_source(#field_names_clap_cloned).unwrap() == ::twelf::reexports::clap::parser::ValueSource::DefaultValue;
                insert_into_map(vals, is_default);
            }
        )*

        let tmp_cfg: #opt_struct_name #struct_gen = ::twelf::reexports::envy::from_iter(map.into_iter())?;
        (::twelf::reexports::serde_json::to_value(tmp_cfg)?,Some(defaulted))
    },};
    let clap_method = quote! { pub fn clap_args() -> Vec<::twelf::reexports::clap::Arg> {
        vec![#(
           ::twelf::reexports::clap::Arg::new(#field_names_clap).long(#field_names_clap).help(#docs)
            .action(if (#fields_is_boolean) {
                ::twelf::reexports::clap::ArgAction::SetTrue
            } else {
                ::twelf::reexports::clap::ArgAction::Set
            })
        ),*]
    }};
    (clap_branch, clap_method)
}

#[cfg(feature = "env")]
fn build_env_branch(opt_struct_name: &Ident, struct_gen: &Generics) -> proc_macro2::TokenStream {
    quote! { ::twelf::Layer::Env(prefix) => match prefix {
        Some(prefix) => {
            let tmp_cfg: #opt_struct_name #struct_gen = ::twelf::reexports::envy::prefixed(prefix).from_env()?;
            (::twelf::reexports::serde_json::to_value(tmp_cfg)?,None)
        },
        None => {
            let tmp_cfg: #opt_struct_name #struct_gen = ::twelf::reexports::envy::from_env()?;
            (::twelf::reexports::serde_json::to_value(tmp_cfg)?,None)
        },
    },}
}

#[cfg(any(
    feature = "json",
    feature = "yaml",
    feature = "toml",
    feature = "ini",
    feature = "dhall"
))]
fn build_shellexpand() -> proc_macro2::TokenStream {
    #[cfg(feature = "shellexpand")]
    quote! { let content = ::twelf::reexports::shellexpand::env(&file_content)? }

    #[cfg(not(feature = "shellexpand"))]
    quote! { let content = file_content }
}

fn build_json_branch() -> proc_macro2::TokenStream {
    #[cfg(feature = "json")]
    let shellexpand = build_shellexpand();
    #[cfg(feature = "json")]
    let json_branch = quote! { ::twelf::Layer::Json(filepath) => {
        let file_content = std::fs::read_to_string(filepath)?;
        #shellexpand;
        (::twelf::reexports::serde_json::from_str(&content)?,None)
    }, };
    #[cfg(not(feature = "json"))]
    let json_branch = quote! {};
    json_branch
}

fn build_toml_branch() -> proc_macro2::TokenStream {
    #[cfg(feature = "toml")]
    let shellexpand = build_shellexpand();
    #[cfg(feature = "toml")]
    let toml_branch = quote! { ::twelf::Layer::Toml(filepath) => {
        let file_content = std::fs::read_to_string(filepath)?;
        // Strip out comments (lines starting with #)
        let file_content = file_content.lines().filter(|line| !line.trim().starts_with("#")).collect::<Vec<_>>().join("\n");

        #shellexpand;
        (::twelf::reexports::toml::from_str(&content)?,None)
    }, };
    #[cfg(not(feature = "toml"))]
    let toml_branch = quote! {};
    toml_branch
}

fn build_yaml_branch() -> proc_macro2::TokenStream {
    #[cfg(feature = "yaml")]
    let shellexpand = build_shellexpand();
    #[cfg(feature = "yaml")]
    let yaml_branch = quote! { ::twelf::Layer::Yaml(filepath) => {
        let file_content = std::fs::read_to_string(filepath)?;
        // Strip out comments (lines starting with #)
        let file_content = file_content.lines().filter(|line| !line.trim().starts_with("#")).collect::<Vec<_>>().join("\n");
        #shellexpand;
        (::twelf::reexports::serde_yaml::from_str(&content)?,None)
    }, };
    #[cfg(not(feature = "yaml"))]
    let yaml_branch = quote! {};
    yaml_branch
}

fn build_dhall_branch() -> proc_macro2::TokenStream {
    #[cfg(feature = "dhall")]
    let shellexpand = build_shellexpand();
    #[cfg(feature = "dhall")]
    let dhall_branch = quote! { ::twelf::Layer::Dhall(filepath) => {
        let file_content = std::fs::read_to_string(filepath)?;
        // Strip out comments (lines starting with --)
        let file_content = file_content.lines().filter(|line| !line.trim().starts_with("--")).collect::<Vec<_>>().join("\n");

        #shellexpand;
        (::twelf::reexports::serde_dhall::from_str(&content).parse()?,None)
    }, };
    #[cfg(not(feature = "dhall"))]
    let dhall_branch = quote! {};
    dhall_branch
}

#[cfg(feature = "ini")]
fn build_ini_branch(opt_struct_name: &Ident, struct_gen: &Generics) -> proc_macro2::TokenStream {
    let shellexpand = build_shellexpand();
    quote! { ::twelf::Layer::Ini(filepath) => {
        let file_content = std::fs::read_to_string(filepath)?;
        // Strip out comments (lines starting with ;)
        let file_content = file_content.lines().filter(|line| !line.trim().starts_with(";")).collect::<Vec<_>>().join("\n");
        #shellexpand;
       let tmp_cfg: #opt_struct_name #struct_gen = ::twelf::reexports::serde_ini::from_str(&content)?;
       (::twelf::reexports::serde_json::to_value(tmp_cfg)?,None)
    }, }
}

fn build_default_trait_branch() -> proc_macro2::TokenStream {
    #[cfg(feature = "default_trait")]
    let default_trait_branch = quote! { ::twelf::Layer::DefaultTrait => (::twelf::reexports::serde_json::to_value(<Self as std::default::Default>::default())?,None), };
    #[cfg(not(feature = "default_trait"))]
    let default_trait_branch = quote! {};
    default_trait_branch
}

fn build_custom_fn_branch() -> proc_macro2::TokenStream {
    #[cfg(feature = "custom_fn")]
    let custom_branch =
        quote! { ::twelf::Layer::CustomFn(custom_fn) => (custom_fn.clone().0(),None), };
    #[cfg(not(feature = "custom_fn"))]
    let custom_branch = quote! {};
    custom_branch
}
