use std::collections::HashMap;

use config::Priority;
use config_derive::config;

fn default_array() -> Vec<String> {
    vec!["first".to_owned(), "second".to_owned()]
}

#[test]
fn simple_types() {
    #[config]
    #[derive(Debug)]
    struct TestCfg {
        test: String,
        another: usize,
    }
    std::env::set_var("ANOTHER", "5");
    let prio = vec![Priority::Json("../test.json".into()), Priority::Env(None)];
    let config = TestCfg::with_priorities(&prio).unwrap();
    assert_eq!(config.test, String::from("from file"));
    assert_eq!(config.another, 5usize);

    let prio = vec![Priority::Env(None), Priority::Json("../test.json".into())];
    let config = TestCfg::with_priorities(&prio).unwrap();
    assert_eq!(config.test, String::from("from file"));
    assert_eq!(config.another, 25usize);
}

#[test]
fn with_array_and_hashmap_string() {
    #[config]
    #[derive(Debug)]
    struct Conf {
        elements: HashMap<String, String>,
        #[serde(default = "default_array")]
        array: Vec<String>,
    }

    // std::env::set_var("TEST", "coucou");
    std::env::set_var("array", "test,test2");
    std::env::set_var("elements", "coucou=toi,hello=you");
    let prio = vec![Priority::Json("../test.json".into()), Priority::Env(None)];
    let config = Conf::with_priorities(&prio).unwrap();
    let mut map = HashMap::new();
    map.insert("coucou".to_string(), "toi".to_string());
    map.insert("hello".to_string(), "you".to_string());

    assert_eq!(config.elements, map);
    let array = vec![String::from("test"), String::from("test2")];
    assert_eq!(config.array, array);

    let mut map = HashMap::new();
    map.insert("key".to_string(), "value".to_string());
    map.insert("key2".to_string(), "value2".to_string());

    let prio = vec![Priority::Env(None), Priority::Json("../test.json".into())];
    let config = Conf::with_priorities(&prio).unwrap();
    assert_eq!(config.elements, map);
    let array = vec![String::from("test"), String::from("test2")];
    assert_eq!(config.array, array);
}

#[test]
fn with_array_and_hashmap_with_default() {
    #[config]
    #[derive(Debug)]
    struct Conf {
        elements: HashMap<String, String>,
        #[serde(default = "default_array")]
        array: Vec<String>,
    }

    std::env::set_var("elements", "coucou=toi,hello=you");
    let prio = vec![Priority::Json("../test.json".into()), Priority::Env(None)];
    let config = Conf::with_priorities(&prio).unwrap();
    let mut map = HashMap::new();
    map.insert("coucou".to_string(), "toi".to_string());
    map.insert("hello".to_string(), "you".to_string());

    assert_eq!(config.elements, map);
    let array = vec![String::from("first"), String::from("second")];
    assert_eq!(config.array, array);
}
