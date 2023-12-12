#![doc = include_str!("../README.md")]

mod error;

#[cfg(any(
    feature = "json",
    feature = "yaml",
    feature = "toml",
    feature = "ini",
    feature = "dhall"
))]
use std::path::PathBuf;

#[doc(hidden)]
pub mod reexports {
    pub use log;
    pub use serde;
    pub use serde_json;
    #[cfg(any(
        feature = "json",
        feature = "yaml",
        feature = "toml",
        feature = "ini",
        feature = "dhall"
    ))]
    pub use shellexpand;

    #[cfg(feature = "clap")]
    pub use clap_rs as clap;
    #[cfg(any(feature = "env", feature = "clap"))]
    pub use envy;
    #[cfg(feature = "dhall")]
    pub use serde_dhall;
    #[cfg(feature = "ini")]
    pub use serde_ini;
    #[cfg(feature = "yaml")]
    pub use serde_yaml;
    #[cfg(feature = "toml")]
    pub use toml_rs as toml;
}

pub use config_derive::config;
pub use error::Error;

/// Layer to configure priority when instantiate configuration.
#[derive(Debug, Clone)]
pub enum Layer {
    /// Env layer taking an optional prefix for environment variables
    #[cfg(feature = "env")]
    Env(Option<String>),
    /// Json layer taking file path to the json file
    #[cfg(feature = "json")]
    Json(PathBuf),
    /// Yaml layer taking file path to the yaml file
    #[cfg(feature = "yaml")]
    Yaml(PathBuf),
    /// Toml layer taking file path to the toml file
    #[cfg(feature = "toml")]
    Toml(PathBuf),
    /// Ini layer taking file path to the ini file
    #[cfg(feature = "ini")]
    Ini(PathBuf),
    /// Dhall layer taking file path to the dhall file
    #[cfg(feature = "dhall")]
    Dhall(PathBuf),
    /// Clap layer taking arguments matches from a clap application
    #[cfg(feature = "clap")]
    Clap(clap_rs::ArgMatches),
    /// Default layer, using std::default::Default trait
    #[cfg(feature = "default_trait")]
    DefaultTrait,
    /// Custom layer, takes any function or closure that outputs a [serde_json::Value].
    #[cfg(feature = "custom_fn")]
    CustomFn(custom_fn::CustomFn),
}

#[cfg(feature = "custom_fn")]
pub mod custom_fn {
    use dyn_clone::{clone_box, DynClone};

    pub struct CustomFn(pub Box<dyn CustomFnTrait>);

    impl<T> From<T> for CustomFn
    where
        T: FnOnce() -> crate::reexports::serde_json::Value + Clone + 'static,
    {
        fn from(func: T) -> Self {
            CustomFn(Box::new(func) as Box<dyn CustomFnTrait>)
        }
    }

    pub trait CustomFnTrait: FnOnce() -> crate::reexports::serde_json::Value + DynClone {}

    impl<T> CustomFnTrait for T where T: Clone + FnOnce() -> crate::reexports::serde_json::Value {}

    impl Clone for CustomFn {
        fn clone(&self) -> Self {
            CustomFn(clone_box(&*self.0) as Box<dyn CustomFnTrait>)
        }
    }

    impl core::fmt::Debug for CustomFn {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "CustomFn")
        }
    }
}
