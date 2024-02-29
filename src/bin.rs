use content_mapp_rs::{get_all_file_paths, index_all_files};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let paths = get_all_file_paths("./test_data")?;
    let results = index_all_files(&paths)?;
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
