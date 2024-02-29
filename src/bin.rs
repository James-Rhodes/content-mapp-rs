use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use content_mapp_rs::indexer::Indexer;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Optional dir to run on
    #[arg(short, long)]
    dir: Option<PathBuf>,

    /// Optional the number of similar files you want to find per file
    #[arg(short, long)]
    n: Option<usize>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Print,
    Serve,
}
fn main() -> Result<()> {
    let cli = Cli::parse();

    let n = cli.n.unwrap_or(5);

    let dir = cli.dir.unwrap_or("./".into());

    let mut indexer = Indexer::new(dir, n)?;
    match cli.command {
        Commands::Print => {
            indexer.index_all_files()?;
            indexer.print_results()?;
        }
        Commands::Serve => todo!(),
    };

    indexer.save_state()?;
    Ok(())
}
