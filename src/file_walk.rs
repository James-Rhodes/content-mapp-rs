use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    time::SystemTime,
};

use anyhow::Result;
use ignore::{DirEntry, Walk};

use crate::cache::NSimilarCache;

pub struct FileData {
    path: PathBuf,
    date_modified: SystemTime,
}

pub struct FileWalker {
    root_dir: PathBuf,
    all_paths: Vec<PathBuf>,
}

impl FileWalker {
    pub fn new<P: AsRef<Path>>(root_dir: P) -> Result<Self> {
        let root_dir = root_dir.as_ref().to_owned();

        let all_paths = Self::get_all_file_paths(&root_dir);
        Ok(FileWalker {
            root_dir,
            all_paths,
        })
    }

    pub fn total_file_count(&self) -> usize {
        self.all_paths.len()
    }

    pub fn all_paths(&mut self) -> Vec<PathBuf> {
        Self::get_all_file_data(&self.root_dir)
            .map(|fd| fd.path)
            .collect()
    }

    pub fn get_all_file_paths<P: AsRef<Path>>(path: &P) -> Vec<PathBuf> {
        Self::get_all_file_data(path).map(|fd| fd.path).collect()
    }

    pub fn get_all_modified_files(&self, cache: &NSimilarCache) -> Vec<PathBuf> {
        let curr_file_data: Vec<FileData> = Self::get_all_file_data(&self.root_dir).collect();
        let mut has_updated = vec![];

        for fd in curr_file_data.iter() {
            let cached_dm = cache.get_path_date_modified(&fd.path);
            // If the file has been modified since the last time the cache was built or the path
            // just doesn't exist in the cache then the file has been modified
            if cached_dm.is_some_and(|dm| fd.date_modified > dm) || cached_dm.is_none() {
                has_updated.push(fd.path.clone());
            }
        }

        // Get all the files that have been removed as well
        let curr_hs: HashSet<PathBuf> = curr_file_data.into_iter().map(|fd| fd.path).collect();
        let removed = cache.get_removed_paths(&curr_hs);
        has_updated.extend(removed);
        has_updated
    }

    fn get_all_file_data<P: AsRef<Path>>(path: &P) -> impl Iterator<Item = FileData> {
        let walker = Walk::new(path);
        walker.filter_map(|e| {
            let e = e.ok();
            if let Some(e) = e {
                if !Self::is_hidden(&e) && !e.file_type()?.is_dir() {
                    let md = e.metadata().ok()?;
                    let date_modified = md.modified().ok()?;

                    let path = e.into_path();
                    return Some(FileData {
                        path,
                        date_modified,
                    });
                }
            }
            None
        })
    }

    fn is_hidden(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| s.starts_with('.') && s != "./")
            .unwrap_or(false)
    }
}
