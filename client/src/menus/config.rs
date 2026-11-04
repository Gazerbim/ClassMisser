use std::collections::HashMap;

use serde_json::Value;

const CONFIG_FILE_PATH: &str = "src/data/config.json";


// Structure to hold the configuration options for the game.
#[derive(Debug, Clone)]
pub struct Config {
    pub options: HashMap<String, Value>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            options: HashMap::new(),
        }
    }

    pub fn load_from_file(&mut self) {
        if let Ok(json_str) = std::fs::read_to_string(CONFIG_FILE_PATH) {
            print!("Configuration loaded: {:?}\n", json_str);
            self.load_from_json(&json_str);
        }
    }

    //load configuration
    pub fn load_from_json(&mut self, json_str: &str) {
        if let Ok(json_value) = serde_json::from_str::<Value>(json_str) {
            if let Some(obj) = json_value.as_object() {
                for (key, value) in obj.iter() {
                    self.options.insert(key.clone(), value.clone());
                    print!("Loaded config option: {} = {}\n", key, value);
                }
            }
        }
    }

    //get a configuration option by key
    pub fn get_option(&self, key: &str) -> Option<&Value> {
        self.options.get(key)
    }

    //set a configuration option by key and value
    pub fn set_option(&mut self, key: String, value: Value) {
        self.options.insert(key, value);
    }
}