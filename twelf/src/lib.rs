//! Twelf is a configuration solution for Rust including 12-Factor support. It is designed with `Layer`s in order to configure different sources and formats to build your configuration. The main goal is to be very simple using the proc macro `twelf::config`. For now it supports :
//!
//! + Default settings (inside your codebase with `#[serde(default = ...)]` coming from [serde](https://serde.rs))
//! + Reading from `TOML`, `YAML`, `JSON`, `DHALL`, `INI` files
//! + Reading from environment variables: it supports `HashMap` structure with `MY_VARIABLE="mykey=myvalue,mykey2=myvalue2"` and also array like `MY_VARIABLE=first,second` thanks to [envy](https://github.com/softprops/envy).
//! + All [serde](https://serde.rs) attributes can be used in your struct to customize your configuration as you wish
//! + Reading your configuration from your command line built with [clap](https://github.com/clap-rs/clap)
//! # Examples
//!
//! ## Simple with JSON and environment variables
//!
//! ```no_run
//! use twelf::{config, Layer};
//!
//! #[config]
//! struct Conf {
//!     test: String,
//!     another: usize,
//! }
//!
//! // Init configuration with layers, each layers override only existing fields
//! let config = Conf::with_layers(&[
//!     Layer::Json("conf.json".into()),
//!     Layer::Env(Some("PREFIX_".to_string()))
//! ]).unwrap();
//! ```
//!
//! ## Example with clap support
//!
//! ```no_run
//! # use twelf::reexports::clap;
//! use twelf::{config, Layer};
//!
//! #[config]
//! struct Conf {
//!     /// Here is an example of documentation which is displayed in clap
//!     test: String,
//!     another: usize,
//! }
//!
//! // Will generate global arguments for each of your fields inside your configuration struct
//! let app = clap::App::new("test").args(&Conf::clap_args());
//!
//! // Init configuration with layers, each layers override only existing fields
//! let config = Conf::with_layers(&[
//!     Layer::Json("conf.json".into()),
//!     Layer::Env(Some("PREFIX_".to_string())),
//!     Layer::Clap(app.get_matches().clone())
//! ]).unwrap();
//!
//! // ... your application code
//! ```
//!
//! ## Use features to improve compile time
//!
//! If you don't want to include useless crates if you just use 2 of all available layers you can use features without default-features, example if you use only yaml and env layer.
//!
//! ```toml
//! [dependencies]
//! twelf = { version = "0.1", default-features = false, features = ["yaml"] }
//! ```
mod error;

use std::path::PathBuf;

#[doc(hidden)]
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

/// Layer to configure priority when instantiate configuration.
#[cfg(feature = "clap")]
#[derive(Debug, Clone)]
pub enum Layer<'a> {
    /// Env layer taking an optional prefix for environment variables
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
