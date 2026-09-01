// src/updater/extract_release_archive.rs

use miette::{Result, miette};
use std::fs::File;
use std::io;
use std::path::Path;
use tempfile::NamedTempFile;

pub fn extract_binary(archive_path: &Path, is_zip: bool) -> Result<NamedTempFile> {
    let file = File::open(archive_path).map_err(|e| miette!("Could not open archive: {}", e))?;
    let mut extracted_bin =
        NamedTempFile::new().map_err(|e| miette!("Failed to create temp bin file: {}", e))?;

    if is_zip {
        use zip::ZipArchive;
        let mut archive =
            ZipArchive::new(file).map_err(|e| miette!("Could not read zip archive: {}", e))?;

        for i in 0..archive.len() {
            let mut entry = archive
                .by_index(i)
                .map_err(|e| miette!("Error reading zip entry: {}", e))?;

            if entry.name().ends_with("woof") || entry.name().ends_with("woof.exe") {
                io::copy(&mut entry, &mut extracted_bin)
                    .map_err(|e| miette!("Failed to extract binary: {}", e))?;
                return Ok(extracted_bin);
            }
        }
    } else {
        use flate2::read::GzDecoder;
        use tar::Archive;
        let tar = GzDecoder::new(file);
        let mut archive = Archive::new(tar);

        for entry in archive
            .entries()
            .map_err(|e| miette!("Could not read archive entries: {}", e))?
        {
            let mut entry = entry.map_err(|e| miette!("Error reading entry: {}", e))?;
            let path = entry
                .path()
                .map_err(|e| miette!("Invalid path in archive: {}", e))?;
            let file_name = path.file_name().unwrap_or_default().to_string_lossy();

            if file_name == "woof" || file_name == "woof.exe" {
                io::copy(&mut entry, &mut extracted_bin)
                    .map_err(|e| miette!("Failed to extract binary: {}", e))?;
                return Ok(extracted_bin);
            }
        }
    }

    Err(miette!("Could not find executable inside the archive."))
}
