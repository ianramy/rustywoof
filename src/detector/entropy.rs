// src/detector/entropy.rs

/// Calculates the Shannon Entropy of a given byte slice.
/// Operates directly on bytes (`&[u8]`) rather than `&str` to avoid UTF-8 validation
/// overhead and to process binary file buffers safely. A higher return value
/// indicates higher cryptographic randomness.
pub fn calculate_shannon_entropy(data: &[u8]) -> f32 {
    if data.is_empty() {
        return 0.0;
    }

    // Stack-allocated array to track frequencies of all 256 possible byte values.
    // This is significantly faster and uses less memory than a HashMap.
    let mut byte_counts = [0usize; 256];
    let mut total_bytes = 0;

    // Tally byte frequencies in a single pass
    for &byte in data {
        byte_counts[byte as usize] += 1;
        total_bytes += 1;
    }

    let mut entropy = 0.0;
    let total_f32 = total_bytes as f32;

    // Apply the Shannon Entropy formula: H = -Σ (p_i * log2(p_i))
    for &count in &byte_counts {
        if count > 0 {
            let probability = count as f32 / total_f32;
            entropy -= probability * probability.log2();
        }
    }

    entropy
}
