use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::RwLock,
    time::SystemTime,
};

use crate::indexer::FileSimilarities;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

pub type Caches = (CompressedByteCache, NSimilarCache);
pub fn load_caches_from_file<P: AsRef<Path>>(path: P) -> Option<Caches> {
    if path.as_ref().exists() {
        let caches = std::fs::read_to_string(path).ok()?;
        return serde_json::from_str(&caches).ok();
    }
    None
}

pub async fn save_caches_to_file<P: AsRef<Path>>(
    path: &P,
    cbc: &CompressedByteCache,
    nsc: &NSimilarCache,
) -> Result<()> {
    let caches = (cbc, nsc);
    let json_string = serde_json::to_string(&caches)?;
    tokio::fs::write(path, json_string).await?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompressedByteCache {
    cache: RwLock<HashMap<PathBuf, usize>>,
}

impl CompressedByteCache {
    pub fn with_capacity(cap: usize) -> Self {
        CompressedByteCache {
            cache: RwLock::new(HashMap::with_capacity(cap)),
        }
    }

    pub fn clear_invalid_paths(&self, _modified_paths: &[PathBuf]) {
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

#[derive(Default, Debug, Serialize, Deserialize)]
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

    pub fn get_path_date_modified(&self, path: &PathBuf) -> Option<SystemTime> {
        Some(self.cache.get(path)?.date_modified)
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

    pub fn get_removed_paths(&self, curr_paths: &HashSet<PathBuf>) -> Vec<PathBuf> {
        let cache_hs: HashSet<PathBuf> = self.cache.keys().cloned().collect();
        cache_hs.difference(curr_paths).cloned().collect()
    }
}
