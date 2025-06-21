// Copyright 2021-2025 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0

use std::{
    io::{Cursor, Read, Write},
    path::Path,
};
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;

/// Utility function to create zipped data from a directory
pub fn zip_directory(dir_path: &std::path::Path) -> anyhow::Result<Vec<u8>> {
    let mut buffer = Vec::new();

    let walkdir = WalkDir::new(dir_path);
    let it = walkdir.into_iter();

    let cursor = Cursor::new(&mut buffer);
    let mut zip = zip::ZipWriter::new(cursor);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::DEFLATE);

    let prefix = Path::new(dir_path);
    let mut file_buffer = Vec::new();
    for entry in it.filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.strip_prefix(prefix).unwrap();

        if path.is_file() {
            zip.start_file_from_path(name, options)?;
            let mut f = std::fs::File::open(path)?;

            f.read_to_end(&mut file_buffer)?;
            zip.write_all(&file_buffer)?;
            file_buffer.clear();
        } else if !name.as_os_str().is_empty() {
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;

    Ok(buffer)
}
