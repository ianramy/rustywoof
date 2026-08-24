// src/updater/archive_extraction.rs

//! Extracts the `woof` executable from downloaded `.tar.gz` or `.zip` release
//! archives into a fresh temporary file.

use miette::{Result, miette};
use std::fs::File;
use std::io;
use tempfile::NamedTempFile;

/// Extracts `woof` or `woof.exe` from a `.tar.gz` archive into a new temp file.
pub(super) fn extract_tar_gz(archive_file: &NamedTempFile) -> Result<NamedTempFile> {
    use flate2::read::GzDecoder;
    use tar::Archive;

    let tar = GzDecoder::new(
        File::open(archive_file).map_err(|e| miette!("Could not open downloaded file: {}", e))?,
    );
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
            let mut extracted_bin = NamedTempFile::new()
                .map_err(|e| miette!("Failed to create temp bin file: {}", e))?;
            io::copy(&mut entry, &mut extracted_bin)
                .map_err(|e| miette!("Failed to extract binary: {}", e))?;
            return Ok(extracted_bin);
        }
    }

    Err(miette!(
        "Could not find the 'woof' executable inside the downloaded .tar.gz archive."
    ))
}

/// Extracts `woof` or `woof.exe` from a `.zip` archive into a new temp file.
pub(super) fn extract_zip(archive_file: &NamedTempFile) -> Result<NamedTempFile> {
    use zip::ZipArchive;

    let file =
        File::open(archive_file).map_err(|e| miette!("Could not open downloaded zip: {}", e))?;
    let mut archive =
        ZipArchive::new(file).map_err(|e| miette!("Could not read zip archive: {}", e))?;

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| miette!("Error reading zip entry: {}", e))?;

        if file.name().ends_with("woof") || file.name().ends_with("woof.exe") {
            let mut extracted_bin = NamedTempFile::new()
                .map_err(|e| miette!("Failed to create temp bin file: {}", e))?;
            io::copy(&mut file, &mut extracted_bin)
                .map_err(|e| miette!("Failed to extract binary: {}", e))?;
            return Ok(extracted_bin);
        }
    }

    Err(miette!(
        "Could not find the 'woof.exe' executable inside the downloaded .zip archive."
    ))
}