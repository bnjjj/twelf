use std::collections::HashMap;

use twelf::reexports::serde::{Deserialize, Serialize};
use twelf::{config, Layer};


#[config]
#[derive(Debug, Default)]
pub struct Settings {
    /// The address to listen on
    
    pub server_url: Option<String>,
    // pub config_file: String,
    pub api_endpoint: Option<String>,
    
    pub profiles: HashMap<String, HashMap<String, String>>,
}

fn main() {
        
        std::env::set_var("APP_INNER", "inner value");
        std::env::set_var("APP_LIST", "value1,value2");
        std::env::set_var(
            "TERRAPHIM_PROFILE_S3",
            "REGION=us-west-2, ENABLE_VIRTUAL_HOST_STYLE=OFF",
        );
        let env_vars = vec![
            // ("TERRAPHIM_PROFILE_S3_REGION", "us-west-1"),
            ("TERRAPHIM_PROFILE_S3_ENABLE_VIRTUAL_HOST_STYLE", "on"),
        ];
        for (k, v) in &env_vars {
            std::env::set_var(k, v);
        }
        let config = Settings::with_layers(&[
            Layer::Toml("./examples/config.toml".into()),
            Layer::Env(Some(String::from("TERRAPHIM_"))),

        ])
        .unwrap();

        assert_eq!(
            config.profiles.get("s3").unwrap().get("region").unwrap(),
            &String::from("us-west-1")
        );
        assert_eq!(
            config.profiles.get("sled").unwrap().get("foo").unwrap(),
            &String::from("bar-3")
        );
        println!("config - {:?}", config);
    }
