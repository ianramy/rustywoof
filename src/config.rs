// src/config.rs

use crate::error::SystemError;
use miette::{Result, WrapErr, IntoDiagnostic};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    pub ignore_paths: Vec<String>,
    pub custom_rules: Vec<CustomRule>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomRule {
    pub name: String,
    pub pattern: String,
}

pub fn load_config() -> Result<Config> {
    let config_path = Path::new(".woof.toml");

    if config_path.exists() {
        let content = fs::read_to_string(config_path).into_diagnostic()?;
        // This is where we construct and use the ConfigParseFailed error!
        let config = toml::from_str(&content).map_err(SystemError::ConfigParseFailed)?;
        Ok(config)
    } else {
        Ok(Config::default())
    }
}

pub fn init_config() -> Result<()> {
    let config = Config {
        ignore_paths: vec![
            "tests/".to_string(),
            "docs/".to_string(),
            "node_modules/".to_string(),
            "target/".to_string(),
        ],
        custom_rules: vec![],
    };

    let toml_content = toml::to_string_pretty(&config).map_err(SystemError::ConfigSerializeFailed)?;

    fs::write(".woof.toml", toml_content)
        .map_err(SystemError::ConfigWriteFailed)
        .wrap_err("Failed to initialize the Watchdog configuration file.")?;

    println!("[INFO] Watchdog perimeter configuration initialized at .woof.toml");
    Ok(())
}
