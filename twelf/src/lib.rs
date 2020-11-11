mod error;

use std::path::PathBuf;

pub mod reexports {
    pub use envy;
    pub use log;
    pub use serde;
    pub use serde_json;

    #[cfg(feature = "clap")]
    pub use clap_rs as clap;
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

#[cfg(feature = "clap")]
#[derive(Debug, Clone)]
pub enum Layer<'a> {
    Env(Option<String>),
    #[cfg(feature = "json")]
    Json(PathBuf),
    #[cfg(feature = "yaml")]
    Yaml(PathBuf),
    #[cfg(feature = "toml")]
    Toml(PathBuf),
    #[cfg(feature = "ini")]
    Ini(PathBuf),
    #[cfg(feature = "dhall")]
    Dhall(PathBuf),
    #[cfg(feature = "clap")]
    Clap(clap_rs::ArgMatches<'a>),
}

#[cfg(not(feature = "clap"))]
#[derive(Debug, Clone)]
pub enum Layer {
    Env(Option<String>),
    #[cfg(feature = "json")]
    Json(PathBuf),
    #[cfg(feature = "yaml")]
    Yaml(PathBuf),
    #[cfg(feature = "toml")]
    Toml(PathBuf),
    #[cfg(feature = "ini")]
    Ini(PathBuf),
    #[cfg(feature = "dhall")]
    Dhall(PathBuf),
}
