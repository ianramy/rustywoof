// tests/updater/test_resolve_github_release.rs

use rustywoof::updater::resolve_github_release::resolve_latest_url;

#[test]
fn test_resolve_latest_returns_none_when_up_to_date() {
    let mut server = mockito::Server::new();

    let mock_response = r#"{
        "tag_name": "v1.0.0",
        "assets": []
    }"#;

    let _m = server
        .mock("GET", "/releases/latest")
        .with_status(200)
        .with_body(mock_response)
        .create();

    let api_url = format!("{}/releases/latest", server.url());
    let result = resolve_latest_url(&api_url, "1.0.0", "x86_64").unwrap();
    assert!(
        result.is_none(),
        "Should return None if current version matches API"
    );
}

#[test]
fn test_resolve_latest_returns_asset_url() {
    let mut server = mockito::Server::new();

    let mock_response = r#"{
        "tag_name": "v2.0.0",
        "assets": [
            { "name": "woof-x86_64-linux", "browser_download_url": "https://example.com/asset" }
        ]
    }"#;

    let _m = server
        .mock("GET", "/releases/latest")
        .with_status(200)
        .with_body(mock_response)
        .create();

    let api_url = format!("{}/releases/latest", server.url());
    let result = resolve_latest_url(&api_url, "1.0.0", "x86_64").unwrap();
    assert_eq!(result.unwrap(), "https://example.com/asset");
}
