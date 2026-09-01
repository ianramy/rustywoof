// src/config/mod.rs

pub mod configuration_schema;
pub mod merge_layered_profiles;
pub mod validate_compiled_rules;

pub use configuration_schema::{Config, CustomRule};

use crate::error::segregate_domain_diagnostics::ConfigDiagnostic;
use miette::{Result, WrapErr};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub struct InitOutcome {
    pub toml_content: String,
    pub imported_files: Vec<String>,
}

pub fn load_config() -> Result<Config> {
    let config_path = Path::new(".woof.toml");
    let config = merge_layered_profiles::resolve(config_path, None, std::env::vars())?;
    validate_compiled_rules::execute_validation(&config)?;
    Ok(config)
}

pub fn init_config() -> Result<InitOutcome> {
    let mut ignore_paths: HashSet<String> = vec![
        "tests/".to_string(),
        "docs/".to_string(),
        "node_modules/".to_string(),
        "target/".to_string(),
        ".git/".to_string(),
        ".venv/".to_string(),
        "venv/".to_string(),
        ".env/".to_string(),
    ]
    .into_iter()
    .collect();

    let mut imported_files = Vec::new();
    let candidates = [".gitignore", ".npmignore", ".dockerignore"];

    for file in candidates {
        if Path::new(file).exists()
            && let Ok(content) = fs::read_to_string(file)
        {
            imported_files.push(file.to_string());
            for line in content.lines() {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('#') {
                    ignore_paths.insert(trimmed.to_string());
                }
            }
        }
    }

    let mut sorted_paths: Vec<String> = ignore_paths.into_iter().collect();
    sorted_paths.sort();

    let config = Config {
        ignore_paths: sorted_paths,
        custom_rules: vec![],
        min_entropy: 3.0,
    };

    let toml_content =
        toml::to_string_pretty(&config).map_err(ConfigDiagnostic::ConfigSerializeFailed)?;

    fs::write(".woof.toml", &toml_content)
        .map_err(ConfigDiagnostic::ConfigWriteFailed)
        .wrap_err("Failed to initialize the Watchdog configuration file.")?;

    Ok(InitOutcome {
        toml_content,
        imported_files,
    })
}
