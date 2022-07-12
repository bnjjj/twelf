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
}
