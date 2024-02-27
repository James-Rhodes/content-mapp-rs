use content_mapp_rs::{
    get_all_file_paths,
    ncd::{self, NormalizedCompressedDistance},
};
use rayon::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let paths = get_all_file_paths("./test_data")?;
    let results: Vec<(String, Vec<NormalizedCompressedDistance>)> = paths
        .par_iter()
        .map(|p| {
            (
                p.to_str().unwrap().to_owned(),
                ncd::get_n_most_similar_files(5, p, &paths).unwrap(),
            )
        })
        .collect();
    for res in results {
        for ncd in res.1 {
            println!(
                "{} -> {} SCORE: {}",
                res.0,
                ncd.path.to_str().unwrap(),
                ncd.ncd_value
            )
        }

        println!("----------------------------------")
    }

    Ok(())
}
