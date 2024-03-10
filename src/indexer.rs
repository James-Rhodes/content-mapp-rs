use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, sync::Arc, time::SystemTime};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    cache::{load_caches_from_file, save_caches_to_file, CompressedByteCache, NSimilarCache},
    file_walk::FileWalker,
    ncd::{self, NormalizedCompressedDistance},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileSimilarities {
    pub n_most_similar: Vec<NormalizedCompressedDistance>,
    pub date_modified: SystemTime,
}

pub struct Indexer {
    n: usize, // The number of similar files we want to compute
    file_walker: FileWalker,
    compressed_byte_cache: Arc<CompressedByteCache>,
    n_similar_cache: NSimilarCache,
    cache_dir: PathBuf,
}

impl Indexer {
    pub fn new(root_dir: impl Into<PathBuf>, n: usize) -> Result<Self> {
        let root_dir: PathBuf = root_dir.into();
        let file_walker = FileWalker::new(root_dir.clone())?;
        let file_count = file_walker.total_file_count();

        let cache_dir = root_dir.join(".content_mapp_rs");
        let (compressed_byte_cache, n_similar_cache) =
            match load_caches_from_file(root_dir.join(".content_mapp_rs")) {
                Some(c) => c,
                None => {
                    let compressed_byte_cache = CompressedByteCache::with_capacity(file_count);

                    let n_similar_cache = NSimilarCache::default();
                    (compressed_byte_cache, n_similar_cache)
                }
            };
        let compressed_byte_cache = Arc::new(compressed_byte_cache);

        Ok(Indexer {
            n,
            file_walker,
            compressed_byte_cache,
            n_similar_cache,
            cache_dir,
        })
    }

    pub fn index_all_files(&mut self) -> Result<()> {
        let paths = self.file_walker.all_paths();

        // Overwrite the current cache as we have completely re-calculated it
        self.n_similar_cache = self.index_files(self.n, &paths)?;

        Ok(())
    }

    pub fn index_modified_files(&mut self) -> Result<()> {
        // Merge in the changes that have occured
        // TODO: The logic is completely broken on this. We still need to reindex every file, its
        // just that we already have the byte length for all of the files that haven't changed
        // We could maybe just calculate the xy combo for the given file too and just a single yx
        // combo for the other file. Something like that idk
        // self.n_similar_cache.merge(self.index_files(self.n, paths)?);

        // NOTE: The below is temporary and can be made more efficient by doing the todo above
        let modified = self
            .file_walker
            .get_all_modified_files(&self.n_similar_cache);

        if !modified.is_empty() {
            println!("Updating File Similarities");
            // Just redo everything if anything has changed
            self.index_all_files()?;
        }

        Ok(())
    }

    fn index_files(&self, n: usize, paths: &[PathBuf]) -> Result<NSimilarCache> {
        let res: HashMap<PathBuf, FileSimilarities> = paths
            .par_iter()
            .map(|p| {
                let n_most_similar =
                    ncd::get_n_most_similar_files_cached(&self.compressed_byte_cache, n, p, paths)?;

                let date_modified = p.metadata()?.modified()?;

                Ok((
                    p.clone(),
                    FileSimilarities {
                        n_most_similar,
                        date_modified,
                    },
                ))
            })
            .collect::<anyhow::Result<HashMap<PathBuf, FileSimilarities>>>()?;

        Ok(NSimilarCache::from(res))
    }

    pub fn print_results(&self) -> Result<()> {
        self.n_similar_cache.print_results()?;
        Ok(())
    }

    pub async fn save_state(&self) -> Result<()> {
        save_caches_to_file(
            &self.cache_dir,
            &self.compressed_byte_cache,
            &self.n_similar_cache,
        )
        .await?;

        Ok(())
    }

    pub fn get_file_sim_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self.n_similar_cache)?)
    }
}
