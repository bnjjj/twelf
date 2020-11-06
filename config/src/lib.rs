mod error;

use std::path::PathBuf;

pub use config_derive::config;
pub use error::Error;

pub mod reexports {
    pub use envy;
    pub use serde;
    pub use serde_json;
    pub use toml;
}

pub enum Priority {
    Env(Option<String>),
    Json(PathBuf),
    Yaml(PathBuf),
    Toml(PathBuf),
}
