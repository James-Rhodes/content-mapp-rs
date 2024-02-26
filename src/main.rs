use std::error::Error;

use ignore::{DirEntry, Walk};
pub mod ncd;

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.') && s != "./")
        .unwrap_or(false)
}

fn get_file_paths(e: Result<DirEntry, ignore::Error>) -> Option<std::path::PathBuf> {
    let e = e.ok();
    if let Some(e) = e {
        if !is_hidden(&e) && !e.file_type()?.is_dir() {
            return Some(e.into_path());
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let walker = Walk::new("./test_data");
    let paths: Vec<std::path::PathBuf> = walker.filter_map(get_file_paths).collect();

    let curr = std::time::Instant::now();
    let res = ncd::get_n_most_similar_files(5, &paths[1], &paths)?;
    println!("Time: {:?}", curr.elapsed());

    println!("NCDS: {:?}", res);
    for ncd in res {
        println!(
            "{} -> {} SCORE: {}",
            paths[1].to_str().unwrap(),
            ncd.path.to_str().unwrap(),
            ncd.ncd_value
        );
    }

    Ok(())
}
