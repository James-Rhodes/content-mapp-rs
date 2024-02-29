use std::path::PathBuf;

use anyhow::Result;
use ignore::{DirEntry, Walk};

pub fn get_all_file_paths(path: &str) -> Result<Vec<PathBuf>> {
    let walker = Walk::new(path);
    Ok(walker
        .filter_map(|e| {
            let e = e.ok();
            if let Some(e) = e {
                if !is_hidden(&e) && !e.file_type()?.is_dir() {
                    return Some(e.into_path());
                }
            }
            None
        })
        .collect())
}

pub fn get_all_modified_files(_path: &str) -> Result<Vec<PathBuf>> {
    todo!()
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.') && s != "./")
        .unwrap_or(false)
}
