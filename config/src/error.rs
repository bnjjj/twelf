#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Envy(envy::Error),
    Json(serde_json::Error),
    Toml(toml::de::Error),
    InvalidFormat,
}

impl From<envy::Error> for Error {
    fn from(err: envy::Error) -> Self {
        Self::Envy(err)
    }
}
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}
impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Self::Toml(err)
    }
}
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
