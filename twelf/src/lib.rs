mod error;

use std::path::PathBuf;

pub mod reexports {
    pub use clap;
    pub use envy;
    pub use serde;
    pub use serde_ini;
    pub use serde_json;
    pub use serde_yaml;
    pub use toml;
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
    Clap(clap::ArgMatches<'a>),
}
