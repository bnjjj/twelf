use std::env::VarError;

use thiserror::Error as ErrorTrait;

/// Error generated when instantiate configuration
#[derive(Debug, ErrorTrait)]
pub enum Error {
    #[error("env lookup error")]
    ShellExpand(#[from] shellexpand::LookupError<VarError>),
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[cfg(any(feature = "env", feature = "clap"))]
    #[error("envy serde error")]
    Envy(#[from] envy::Error),
    #[error("json serde error")]
    Json(#[from] serde_json::Error),
    #[cfg(feature = "toml")]
    #[error("toml serde error")]
    Toml(#[from] toml_rs::de::Error),
    #[cfg(feature = "yaml")]
    #[error("yaml serde error")]
    Yaml(#[from] serde_yaml::Error),
    #[cfg(feature = "ini")]
    #[error("ini serde error")]
    Ini(#[from] serde_ini::de::Error),
    #[cfg(feature = "dhall")]
    #[error("dhall serde error")]
    Dhall(#[from] serde_dhall::Error),
    #[error("invalid format")]
    InvalidFormat,
    #[error("deserialize error: {0}")]
    Deserialize(String),
}
