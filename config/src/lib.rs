use std::{collections::HashMap, path::PathBuf};

use config_derive::config;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{value::RawValue, Map, Value};
use std::iter::FromIterator;

pub mod reexports {
    pub use envy;
    pub use serde;
    pub use serde_json;
    pub use toml;
}

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

pub enum Priority {
    Env(Option<String>),
    Json(PathBuf),
    Yaml(PathBuf),
    Toml(PathBuf),
}

impl Priority {
    pub fn parse<T>(&self) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let res = match self {
            Priority::Env(prefix) => match prefix {
                Some(prefix) => envy::prefixed(prefix).from_env(),
                None => envy::from_env(),
            }?,
            Priority::Json(filepath) => serde_json::from_reader(std::fs::File::open(filepath)?)?,
            Priority::Toml(filepath) => toml::from_str(&std::fs::read_to_string(filepath)?)?,
            Priority::Yaml(_) => todo!(),
        };

        Ok(res)
    }
}

pub struct Builder {
    priorities: Vec<Priority>,
}

impl Builder {
    pub fn build<T>(&self) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let mut res: HashMap<String, Value> = HashMap::new();
        for prio in &self.priorities {
            let extension: Value = prio.parse()?;
            // println!("extension --- {:?}", extension);
            res.extend(
                extension
                    .as_object()
                    .ok_or_else(|| Error::InvalidFormat)?
                    .to_owned()
                    .into_iter(),
            );
        }

        serde_json::from_value(Value::Object(Map::from_iter(res.into_iter()))).map_err(Error::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Map, Value};
    use std::iter::FromIterator;

    #[derive(Debug, Deserialize)]
    struct ExpCfg {
        test: String,
        another: usize,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct OptionExpCfg {
        test: Option<String>,
        another: Option<usize>,
    }

    #[test]
    fn test_envy() {
        std::env::set_var("TEST", "in env");
        std::env::set_var("ANOTHER", "5");
        // TODO: do this in proc_macro to generate OptionStruct
        let test_cfg: OptionExpCfg = envy::from_env().unwrap();
        let test_cfg_val = serde_json::to_value(test_cfg).unwrap();

        println!("config --- {:?}", test_cfg_val);
    }

    #[test]
    fn test_serde_value() {
        #[derive(Debug, Deserialize)]
        struct TestCfg {
            test: String,
            another: usize,
        }
        std::env::set_var("TEST", "in env");
        let mut test_cfg: HashMap<String, Value> = envy::from_env().unwrap();
        let file_content = std::fs::read_to_string("../test.json").unwrap();

        let cfg: HashMap<String, serde_json::Value> = serde_json::from_str(&file_content).unwrap();

        test_cfg.extend(cfg);

        println!("---- {:?}", test_cfg.get("test"));

        let cfg: TestCfg =
            serde_json::from_value(Value::Object(Map::from_iter(test_cfg.into_iter()))).unwrap();

        println!("config --- {:?}", cfg);
    }

    // #[test]
    // fn test_builder() {
    //     #[derive(Debug)]
    //     struct TestCfg {
    //         test: String,
    //         another: usize,
    //     }
    //     std::env::set_var("TEST", "in env");
    //     std::env::set_var("ANOTHER", "5");

    //     let builder = Builder {
    //         priorities: vec![Priority::Json("../test.json".into()), Priority::Env(None)],
    //     };

    //     let cfg: TestCfg = builder.build().unwrap();

    //     println!("----- cfg = {:?}", cfg);
    // }
}
