use anyhow::Result;
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

// use ncd::NormalizedCompressedDistance;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::ncd::{self, NormalizedCompressedDistance};

pub struct FileSimilarities {
    pub file_path: PathBuf,
    pub n_most_similar: Vec<NormalizedCompressedDistance>,
}

pub fn index_all_files(paths: &Vec<PathBuf>, n: usize) -> Result<Vec<FileSimilarities>> {
    let state = vec![None; paths.len()];
    let state: Arc<RwLock<Vec<Option<usize>>>> = Arc::new(RwLock::new(state));
    paths
        .par_iter()
        .map(|p| {
            let file_path: PathBuf = p.to_str().unwrap().to_owned().into();
            let n_most_similar = ncd::get_n_most_similar_files_cached(n, p, paths, &state)?;

            Ok(FileSimilarities {
                file_path,
                n_most_similar,
            })
        })
        .collect()
}
