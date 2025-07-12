use confy::ConfyError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const CONFIG_NAME: &str = "config";

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    extra: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        let mut extra = HashMap::new();
        extra.insert("Hello".to_string(), "echo Hello, World!".to_string());
        extra.insert("PingGoogle".to_string(), "ping 8.8.8.8".to_string());

        Self { extra }
    }
}

impl Config {
    pub fn load() -> Result<Self, ConfyError> {
        confy::load(APP_NAME, CONFIG_NAME)
    }

    pub fn store(&self) -> Result<(), ConfyError> {
        confy::store(APP_NAME, CONFIG_NAME, self)
    }

    pub fn keys(&self) -> Vec<String> {
        self.extra.keys().cloned().collect::<Vec<String>>()
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.extra.get(key).cloned()
    }

    pub fn insert(&mut self, name: impl ToString, command: impl ToString) {
        self.extra.insert(name.to_string(), command.to_string());
    }

    pub fn remove(&mut self, name: impl ToString) {
        self.extra.remove(&name.to_string());
    }
}
