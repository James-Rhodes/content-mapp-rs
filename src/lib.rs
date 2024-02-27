pub mod ncd;

use std::{error::Error, path::PathBuf};

use ignore::{DirEntry, Walk};

pub fn get_all_file_paths(path: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
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

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.') && s != "./")
        .unwrap_or(false)
}
