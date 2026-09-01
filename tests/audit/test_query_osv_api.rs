// tests/audit/test_query_osv_api.rs

use rustywoof::audit::manage_osv_cache;
use rustywoof::audit::query_osv_api::batch_query_osv;
use std::fs;

#[test]
fn test_batch_query_osv_clean_dependencies() {
    let mut server = mockito::Server::new();

    let _m = server
        .mock("POST", "/querybatch")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"results": [{}]}"#)
        .create();

    let deps = vec![(
        "safe-package".to_string(),
        "1.0.0".to_string(),
        "npm".to_string(),
    )];

    let cache_key = manage_osv_cache::generate_cache_key(&deps);
    let cache_path = manage_osv_cache::get_cache_path(cache_key);
    let _ = fs::remove_file(&cache_path);

    let result = batch_query_osv(&deps, Some(&server.url()), false, false, None, false)
        .expect("Failed to query OSV API");

    assert!(result, "Dependencies should be marked as clean");
}

#[test]
fn test_batch_query_osv_vulnerable_dependencies() {
    let mut server = mockito::Server::new();

    let querybatch_response = r#"{
        "results": [
            {
                "vulns": [
                    {
                        "id": "GHSA-1234",
                        "aliases": ["CVE-2023-1234"],
                        "severity": [{"type": "CVSS_V3", "score": "CVSS:3.1/AV:N/AC:H/PR:N/UI:N/S:U/C:N/I:N/A:H"}],
                        "affected": [
                            {
                                "package": { "name": "bad-package", "ecosystem": "npm" },
                                "ranges": [
                                    { "events": [ { "fixed": "1.0.2" } ] }
                                ]
                            }
                        ]
                    }
                ]
            }
        ]
    }"#;

    let _m_batch = server
        .mock("POST", "/querybatch")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(querybatch_response)
        .create();

    let _m_query = server
        .mock("POST", "/query")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(querybatch_response)
        .create();

    let deps = vec![(
        "bad-package".to_string(),
        "1.0.1".to_string(),
        "npm".to_string(),
    )];

    let cache_key = manage_osv_cache::generate_cache_key(&deps);
    let cache_path = manage_osv_cache::get_cache_path(cache_key);
    let _ = fs::remove_file(&cache_path);

    let result = batch_query_osv(&deps, Some(&server.url()), false, false, None, false)
        .expect("Failed to query OSV API");

    assert!(!result, "Dependencies should be marked as vulnerable");
}
