use anyhow::Result;
use content_mapp_rs::indexer::Indexer;

fn main() -> Result<()> {
    let dir = "./test_data";
    let mut indexer = Indexer::new(dir, 5)?;
    indexer.index_all_files()?;
    indexer.print_results()?;
    Ok(())
}
