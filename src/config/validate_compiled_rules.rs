// src/config/validate_compiled_rules.rs

use crate::config::configuration_schema::Config;
use crate::error::segregate_domain_diagnostics::ConfigDiagnostic;
use regex::Regex;

pub fn execute_validation(config: &Config) -> Result<(), ConfigDiagnostic> {
    if config.min_entropy < 0.0 || config.min_entropy > 8.0 {
        return Err(ConfigDiagnostic::ConfigValidationFailed(format!(
            "Shannon entropy threshold ({}) must be between 0.0 and 8.0",
            config.min_entropy
        )));
    }

    for rule in &config.custom_rules {
        if let Err(e) = Regex::new(&rule.pattern) {
            return Err(ConfigDiagnostic::ConfigValidationFailed(format!(
                "Custom rule '{}' contains an invalid regex pattern: {}",
                rule.name, e
            )));
        }
    }

    Ok(())
}
