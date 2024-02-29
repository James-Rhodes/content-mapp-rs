pub mod ncd;

use std::{
    error::Error,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use ignore::{DirEntry, Walk};
use ncd::NormalizedCompressedDistance;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

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

pub type SimilarFiles = Vec<(String, Vec<NormalizedCompressedDistance>)>;
pub fn index_all_files(paths: &Vec<PathBuf>) -> Result<SimilarFiles, Box<dyn Error>> {
    let state = vec![None; paths.len()];
    let mut state: Arc<RwLock<Vec<Option<usize>>>> = Arc::new(RwLock::new(state));
    Ok(paths
        .par_iter()
        .map(|p| {
            (
                p.to_str().unwrap().to_owned(),
                ncd::get_n_most_similar_files_cached(5, p, paths, &*state).unwrap(),
            )
        })
        .collect())
}
