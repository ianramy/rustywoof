// src/config/merge_layered_profiles.rs

use crate::config::configuration_schema::Config;
use crate::error::segregate_domain_diagnostics::ConfigDiagnostic;
use miette::{IntoDiagnostic, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn resolve<I>(local_path: &Path, global_path: Option<&Path>, env_vars: I) -> Result<Config>
where
    I: IntoIterator<Item = (String, String)>,
{
    let mut merged_value: toml::Value =
        toml::from_str("").expect("Empty string is inherently a valid empty TOML table");

    if let Some(g_path) = global_path.filter(|p| p.exists()) {
        let content = fs::read_to_string(g_path).into_diagnostic()?;
        merged_value = toml::from_str(&content).map_err(ConfigDiagnostic::ConfigParseFailed)?;
    }

    if local_path.exists() {
        let content = fs::read_to_string(local_path).into_diagnostic()?;
        let local_value: toml::Value =
            toml::from_str(&content).map_err(ConfigDiagnostic::ConfigParseFailed)?;

        if let (Some(base_table), Some(local_table)) =
            (merged_value.as_table_mut(), local_value.as_table())
        {
            for (k, v) in local_table {
                base_table.insert(k.clone(), v.clone());
            }
        } else if merged_value.as_table().is_none() {
            merged_value = local_value;
        }
    }

    let merged_str =
        toml::to_string(&merged_value).map_err(ConfigDiagnostic::ConfigSerializeFailed)?;
    let mut config: Config =
        toml::from_str(&merged_str).map_err(ConfigDiagnostic::ConfigParseFailed)?;

    let env_map: HashMap<String, String> = env_vars.into_iter().collect();
    if let Some(val) = env_map.get("WOOF_MIN_ENTROPY") {
        config.min_entropy = val.parse::<f32>().unwrap_or(config.min_entropy);
    }

    Ok(config)
}
