// tests/updater/test_extract_release_archive.rs

use rustywoof::updater::extract_release_archive::extract_binary;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_extract_fails_gracefully_on_invalid_archive() {
    let mut dummy = NamedTempFile::new().unwrap();
    dummy.write_all(b"not an archive").unwrap();

    let result = extract_binary(dummy.path(), false);
    assert!(
        result.is_err(),
        "Should return diagnostic error on corrupt archive, not panic"
    );
}
