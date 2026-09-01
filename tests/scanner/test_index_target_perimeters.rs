// tests/scanner/test_index_target_perimeters.rs

use rustywoof::scanner::index_target_perimeters;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_indexes_directory_and_calculates_bytes() {
    let dir = tempdir().unwrap();
    let file_path1 = dir.path().join("file1.txt");
    let file_path2 = dir.path().join("file2.txt");

    fs::write(&file_path1, "Hello").unwrap(); // 5 bytes
    fs::write(&file_path2, "World!").unwrap(); // 6 bytes

    let index = index_target_perimeters::index_directory(dir.path().to_str().unwrap(), &[]);

    assert_eq!(
        index.total_files, 2,
        "Failed to count the correct number of files"
    );
    assert_eq!(
        index.total_bytes, 11,
        "Failed to sum the correct number of bytes"
    );
}
