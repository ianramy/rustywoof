// src/scanner/index_target_perimeters.rs

use ignore::WalkBuilder;
use std::collections::HashMap;
use std::path::Component;

pub const MAX_FILE_SIZE_BYTES: u64 = 15 * 1024 * 1024; // 15 MB

pub struct IndexMetrics {
    pub total_bytes: u64,
    pub total_files: u64,
    pub top_folders: String,
}

pub fn index_directory(target_path: &str, ignore_paths: &[String]) -> IndexMetrics {
    let mut builder = WalkBuilder::new(target_path);
    builder
        .hidden(false)
        .filter_entry(|e| e.file_name() != ".git")
        .ignore(false);

    if !ignore_paths.is_empty() {
        let mut overrides = ignore::overrides::OverrideBuilder::new(target_path);
        for path in ignore_paths {
            let _ = overrides.add(&format!("!{}", path));
        }
        if let Ok(ov) = overrides.build() {
            builder.overrides(ov);
        }
    }

    let walker = builder.build();
    let mut total_bytes = 0u64;
    let mut expected_files = 0u64;
    let mut folder_sizes: HashMap<String, u64> = HashMap::new();

    for entry in walker.flatten() {
        if entry.file_type().is_some_and(|ft| ft.is_file())
            && let Ok(metadata) = entry.metadata()
        {
            let size = metadata.len();
            if size > MAX_FILE_SIZE_BYTES {
                continue;
            }
            total_bytes += size;
            expected_files += 1;

            if let Some(Component::Normal(folder)) = entry.path().components().next() {
                let folder_name = folder.to_string_lossy().to_string();
                *folder_sizes.entry(folder_name).or_insert(0) += size;
            }
        }
    }

    let mut sorted_folders: Vec<_> = folder_sizes.into_iter().collect();
    sorted_folders.sort_by_key(|b| std::cmp::Reverse(b.1));

    let top_folders = sorted_folders
        .into_iter()
        .take(3)
        .map(|(name, size)| format!("{}/ ({:.1} MiB)", name, size as f64 / 1_048_576.0))
        .collect::<Vec<_>>()
        .join(" | ");

    IndexMetrics {
        total_bytes,
        total_files: expected_files,
        top_folders,
    }
}
