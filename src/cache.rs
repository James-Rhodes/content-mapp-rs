use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::RwLock,
};

use crate::indexer::FileSimilarities;
use anyhow::{Context, Result};

pub fn load_caches_from_file<P: AsRef<Path>>(
    path: P,
) -> Option<(CompressedByteCache, NSimilarCache)> {
    if path.as_ref().exists() {
        todo!()
    }
    None
}

pub fn save_caches_to_file<P: AsRef<Path>>(
    path: P,
) -> Option<(CompressedByteCache, NSimilarCache)> {
    todo!()
}

// TODO: Update this
pub struct CompressedByteCache {
    cache: RwLock<HashMap<PathBuf, usize>>,
}

impl CompressedByteCache {
    pub fn with_capacity(cap: usize) -> Self {
        CompressedByteCache {
            cache: RwLock::new(HashMap::with_capacity(cap)),
        }
    }

    pub fn clear_invalid_paths(&self, modified_paths: &[PathBuf]) {
        todo!()
        // Remove any paths from cache that no longer exist
        //
        // let mut cache = HashMap::with_capacity(existing_paths.len());
        // for path in existing_paths {
        //     let curr_cache = self.cache.read().unwrap();
        //     if let Some(cnt) = curr_cache.get(path) {
        //         cache.insert(path.clone(), *cnt);
        //     }
        // }
        // self.cache = RwLock::new(cache);
    }

    pub fn get_or<F>(&self, path: &PathBuf, f: F) -> Result<usize>
    where
        F: FnOnce() -> Result<usize>,
    {
        {
            let cache = self.cache.read().unwrap();
            if let Some(cnt) = cache.get(path) {
                return Ok(*cnt);
            }
        }

        let result = f()?;
        self.cache.write().unwrap().insert(path.clone(), result);

        Ok(result)
    }
}

#[derive(Default)]
pub struct NSimilarCache {
    cache: HashMap<PathBuf, FileSimilarities>,
}

impl NSimilarCache {
    pub fn merge(&mut self, other: NSimilarCache) {
        self.cache.extend(other.cache);
    }

    pub fn from(hm: HashMap<PathBuf, FileSimilarities>) -> Self {
        NSimilarCache { cache: hm }
    }

    pub fn print_results(&self) -> Result<()> {
        for (path, fsim) in &self.cache {
            for ncd in &fsim.n_most_similar {
                println!(
                    "{} -> {} SCORE: {}",
                    path.to_str().context("Cannot convert file path to str")?,
                    ncd.file_path
                        .to_str()
                        .context("Cannot convert similar file path to str")?,
                    ncd.ncd_value
                )
            }

            println!("----------------------------------")
        }
        Ok(())
    }
}
