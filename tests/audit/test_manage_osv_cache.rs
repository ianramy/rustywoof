// tests/audit/test_manage_osv_cache.rs

use rustywoof::audit::manage_osv_cache;

#[test]
fn test_cache_key_generation_is_deterministic() {
    let deps = vec![(
        "serde".to_string(),
        "1.0.0".to_string(),
        "crates.io".to_string(),
    )];
    let key1 = manage_osv_cache::generate_cache_key(&deps);
    let key2 = manage_osv_cache::generate_cache_key(&deps);
    assert_eq!(key1, key2, "Cache key hashing must be deterministic");
}

#[test]
fn test_cache_read_write_lifecycle() {
    let deps = vec![("mock-pkg".to_string(), "1.0".to_string(), "npm".to_string())];
    let key = manage_osv_cache::generate_cache_key(&deps);
    let cache_file = manage_osv_cache::get_cache_path(key);

    let data = r#"{"test": true}"#;
    manage_osv_cache::write_cache(&cache_file, data).expect("Failed to write to OSV cache");

    let read_data = manage_osv_cache::read_cache_if_valid(&cache_file, 3600)
        .expect("Failed to read valid cache");
    assert_eq!(read_data, data);

    let expired = manage_osv_cache::read_cache_if_valid(&cache_file, 0);
    assert!(
        expired.is_none(),
        "Cache should be invalidated when past its TTL"
    );
}
