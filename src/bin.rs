use anyhow::{Context, Result};
use content_mapp_rs::{file_walk::get_all_file_paths, indexer::index_all_files};

fn main() -> Result<()> {
    let paths = get_all_file_paths("./test_data")?;
    let results = index_all_files(&paths, 5)?;
    for res in results {
        for ncd in res.n_most_similar {
            println!(
                "{} -> {} SCORE: {}",
                res.file_path
                    .to_str()
                    .context("Cannot convert file path to str")?,
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
