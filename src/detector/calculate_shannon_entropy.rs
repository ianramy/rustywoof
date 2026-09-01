// src/detector/calculate_shannon_entropy.rs

pub fn calculate_entropy(data: &[u8]) -> f32 {
    if data.is_empty() {
        return 0.0;
    }

    let mut byte_counts = [0usize; 256];
    for &byte in data {
        byte_counts[byte as usize] += 1;
    }

    let mut entropy = 0.0;
    let total_f32 = data.len() as f32;

    for &count in &byte_counts {
        if count > 0 {
            let probability = count as f32 / total_f32;
            entropy -= probability * probability.log2();
        }
    }

    entropy
}

pub fn find_high_entropy_windows(data: &[u8], window_size: usize, threshold: f32) -> Vec<usize> {
    let mut offsets = Vec::new();
    if data.len() < window_size {
        return offsets;
    }

    for i in 0..=(data.len() - window_size) {
        let window = &data[i..i + window_size];
        if calculate_entropy(window) >= threshold {
            offsets.push(i);
        }
    }

    offsets
}
