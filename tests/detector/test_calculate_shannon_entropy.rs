// tests/detector/test_calculate_shannon_entropy.rs

use rustywoof::detector::calculate_shannon_entropy;

#[test]
fn test_calculates_entropy_correctly() {
    let low_entropy = b"aaaaa";
    let high_entropy = b"aB3!x9Qz";

    let h_low = calculate_shannon_entropy::calculate_entropy(low_entropy);
    let h_high = calculate_shannon_entropy::calculate_entropy(high_entropy);

    assert_eq!(h_low, 0.0, "Uniform data should have 0.0 entropy");
    assert!(h_high > 2.5, "Random data should have high entropy");
}

#[test]
fn test_sliding_window_locates_high_entropy_regions() {
    let buffer = b"let a=0; let b=0; let key='zQ9#xV!2wKpR7$Lm'; let c=0; let d=0;";
    let secret = b"zQ9#xV!2wKpR7$Lm";

    let expected_offset = buffer
        .windows(secret.len())
        .position(|w| w == secret)
        .unwrap();

    let regions = calculate_shannon_entropy::find_high_entropy_windows(buffer, 16, 3.8);

    assert!(
        !regions.is_empty(),
        "Failed to locate any high entropy regions"
    );

    assert!(
        regions.contains(&expected_offset),
        "The exact secret window offset ({}) was not flagged. Flagged regions: {:?}",
        expected_offset,
        regions
    );
}
