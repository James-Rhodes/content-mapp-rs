use std::path::{Path, PathBuf};

use anyhow::Result;
use ignore::{DirEntry, Walk};

pub struct FileWalker {
    root_dir: PathBuf,
    all_paths: Vec<PathBuf>,
}

impl FileWalker {
    pub fn new<P: AsRef<Path>>(root_dir: P) -> Result<Self> {
        Ok(FileWalker {
            root_dir: root_dir.as_ref().to_owned(),
            all_paths: Self::get_all_file_paths(root_dir)?,
        })
    }

    pub fn total_file_count(&self) -> usize {
        self.all_paths.len()
    }

    pub fn all_paths(&self) -> &[PathBuf] {
        self.all_paths.as_ref()
    }

    pub fn get_all_file_paths<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>> {
        let walker = Walk::new(path);
        Ok(walker
            .filter_map(|e| {
                let e = e.ok();
                if let Some(e) = e {
                    if !Self::is_hidden(&e) && !e.file_type()?.is_dir() {
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
}
