use std::collections::HashMap;

use config_derive::config;
use twelf::Layer;

fn default_array() -> Vec<String> {
    vec!["first".to_owned(), "second".to_owned()]
}

const DHALL_TEST_FILE: &str = "./tests/fixtures/test.dhall";

#[test]
fn dhall_simple_types() {
    #[config]
    #[derive(Debug, Default)]
    struct TestCfg {
        test: String,
        another: usize,
    }
    std::env::set_var("ANOTHER", "5");
    let prio = vec![Layer::Dhall(DHALL_TEST_FILE.into()), Layer::Env(None)];
    let config = TestCfg::with_layers(&prio).unwrap();
    assert_eq!(config.test, String::from("from file"));
    assert_eq!(config.another, 5usize);

    let prio = vec![Layer::Env(None), Layer::Dhall(DHALL_TEST_FILE.into())];
    let config = TestCfg::with_layers(&prio).unwrap();
    assert_eq!(config.test, String::from("from file"));
    assert_eq!(config.another, 25usize);
}

#[test]
fn dhall_simple_with_prefix() {
    #[config]
    #[derive(Debug, Default)]
    struct Conf {
        elements_def: HashMap<String, String>,
        #[serde(default = "default_array")]
        array_def: Vec<String>,
    }

    std::env::set_var("APP_ELEMENTS_DEF", "coucou=toi,hello=you");
    let prio = vec![
        Layer::Dhall(DHALL_TEST_FILE.into()),
        Layer::Env(Some("APP_".to_string())),
    ];
    let config = Conf::with_layers(&prio).unwrap();
    let mut map = HashMap::new();
    map.insert("coucou".to_string(), "toi".to_string());
    map.insert("hello".to_string(), "you".to_string());

    assert_eq!(config.elements_def, map);
    let array = vec![String::from("first"), String::from("second")];
    assert_eq!(config.array_def, array);
}

#[test]
fn dhall_simple_with_option() {
    #[config]
    #[derive(Debug, Default)]
    struct Conf {
        elements_def: Option<HashMap<String, String>>,
        array_def: Option<Vec<String>>,
    }

    std::env::set_var("APPBIS_ELEMENTS_DEF", "coucou=toi,hello=you");
    let prio = vec![
        Layer::Dhall(DHALL_TEST_FILE.into()),
        Layer::Env(Some("APPBIS_".to_string())),
    ];
    let config = Conf::with_layers(&prio).unwrap();
    let mut map = HashMap::new();
    map.insert("coucou".to_string(), "toi".to_string());
    map.insert("hello".to_string(), "you".to_string());

    assert_eq!(config.elements_def, Some(map));
    assert_eq!(config.array_def, None);
}

#[test]
fn dhall_with_array_and_hashmap_string() {
    #[config]
    #[derive(Debug, Default)]
    struct Conf {
        elements: HashMap<String, String>,
        #[serde(default = "default_array")]
        array: Vec<String>,
    }

    std::env::set_var("ARRAY", "test,test2");
    std::env::set_var("ELEMENTS", "coucou=toi,hello=you");
    let prio = vec![Layer::Dhall(DHALL_TEST_FILE.into()), Layer::Env(None)];
    let config = Conf::with_layers(&prio).unwrap();
    let mut map = HashMap::new();
    map.insert("coucou".to_string(), "toi".to_string());
    map.insert("hello".to_string(), "you".to_string());

    assert_eq!(config.elements, map);
    let array = vec![String::from("test"), String::from("test2")];
    assert_eq!(config.array, array);

    let mut map = HashMap::new();
    map.insert("key".to_string(), "value".to_string());
    map.insert("key2".to_string(), "value2".to_string());

    let prio = vec![Layer::Env(None), Layer::Dhall(DHALL_TEST_FILE.into())];
    let config = Conf::with_layers(&prio).unwrap();
    assert_eq!(config.elements, map);
    let array = vec![String::from("test"), String::from("test2")];
    assert_eq!(config.array, array);
}

#[test]
fn dhall_with_array_and_hashmap_with_default() {
    #[config]
    #[derive(Debug, Default)]
    struct Conf {
        elements_def: HashMap<String, String>,
        #[serde(default = "default_array")]
        array_def: Vec<String>,
    }

    std::env::set_var("ELEMENTS_DEF", "coucou=toi,hello=you");
    let prio = vec![Layer::Dhall(DHALL_TEST_FILE.into()), Layer::Env(None)];
    let config = Conf::with_layers(&prio).unwrap();
    let mut map = HashMap::new();
    map.insert("coucou".to_string(), "toi".to_string());
    map.insert("hello".to_string(), "you".to_string());

    assert_eq!(config.elements_def, map);
    let array = vec![String::from("first"), String::from("second")];
    assert_eq!(config.array_def, array);
}
