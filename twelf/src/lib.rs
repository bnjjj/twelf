mod error;

use std::path::PathBuf;

pub mod reexports {
    pub use clap_rs as clap;
    pub use envy;
    pub use log;
    pub use serde;
    pub use serde_dhall;
    pub use serde_ini;
    pub use serde_json;
    pub use serde_yaml;
    pub use toml_rs as toml;
}

pub use config_derive::config;
pub use error::Error;

#[derive(Debug, Clone)]
pub enum Layer<'a> {
    Env(Option<String>),
    Json(PathBuf),
    Yaml(PathBuf),
    Toml(PathBuf),
    Ini(PathBuf),
    Dhall(PathBuf),
    Clap(clap_rs::ArgMatches<'a>),
}
