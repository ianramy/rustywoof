// src/audit/manage_osv_cache.rs

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::time::SystemTime;
use std::{env, fs};

pub fn generate_cache_key(dependencies: &[(String, String, String)]) -> u64 {
    let mut hasher = DefaultHasher::new();
    dependencies.hash(&mut hasher);
    hasher.finish()
}

pub fn get_cache_path(key: u64) -> PathBuf {
    let cache_dir = env::temp_dir().join("woof_osv_cache");
    let _ = fs::create_dir_all(&cache_dir);
    cache_dir.join(format!("osv_results_{}.json", key))
}

pub fn read_cache_if_valid(path: &PathBuf, max_age_secs: u64) -> Option<String> {
    if path.exists()
        && let Ok(metadata) = fs::metadata(path)
        && let Ok(modified) = metadata.modified()
        && let Ok(duration) = SystemTime::now().duration_since(modified)
        && duration.as_secs() < max_age_secs
    {
        return fs::read_to_string(path).ok();
    }
    None
}

pub fn write_cache(path: &PathBuf, data: &str) -> std::io::Result<()> {
    fs::write(path, data)
}
