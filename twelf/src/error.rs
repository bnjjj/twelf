use thiserror::Error as ErrorTrait;

#[derive(Debug, ErrorTrait)]
pub enum Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("envy serde error")]
    Envy(#[from] envy::Error),
    #[error("json serde error")]
    Json(#[from] serde_json::Error),
    #[error("toml serde error")]
    Toml(#[from] toml::de::Error),
    #[error("yaml serde error")]
    Yaml(#[from] serde_yaml::Error),
    #[error("ini serde error")]
    Ini(#[from] serde_ini::de::Error),
    #[error("invalid format")]
    InvalidFormat,
    #[error("deserialize error: {0}")]
    Deserialize(String),
}
