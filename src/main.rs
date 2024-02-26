use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::prelude::*;

use std::error::Error;

use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.') && s != "./")
        .unwrap_or(false)
}

fn get_file_paths(e: Result<DirEntry, walkdir::Error>) -> Option<std::path::PathBuf> {
    let e = e.ok();
    if let Some(e) = e {
        if !is_hidden(&e) && !e.file_type().is_dir() {
            return Some(e.into_path());
        }
    }
    None
}

fn get_compressed_byte_count(buf: &[u8]) -> Result<usize, Box<dyn Error>> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(buf)?;
    let compressed_bytes = e.finish()?;
    Ok(compressed_bytes.len())
}

fn main() -> Result<(), Box<dyn Error>> {
    let walker = WalkDir::new("./").into_iter();
    let paths: Vec<std::path::PathBuf> = walker.filter_map(get_file_paths).collect();
    let curr = std::time::Instant::now();
    for path in paths {
        let content = std::fs::read(&path)?;
        println!(
            "Path {:?} uncompressed len: {}, compressed len: {}",
            path,
            content.len(),
            get_compressed_byte_count(&content)?
        );
    }
    println!("Time: {:?}", curr.elapsed());

    Ok(())
}
