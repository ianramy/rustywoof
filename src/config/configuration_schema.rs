// src/config/configuration_schema.rs

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default)]
    pub ignore_paths: Vec<String>,
    #[serde(default)]
    pub custom_rules: Vec<CustomRule>,
    #[serde(default = "default_entropy")]
    pub min_entropy: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomRule {
    pub name: String,
    pub pattern: String,
}

fn default_entropy() -> f32 {
    3.0
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ignore_paths: vec![],
            custom_rules: vec![],
            min_entropy: default_entropy(),
        }
    }
}
