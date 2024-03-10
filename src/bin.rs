use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use clap::{Parser, Subcommand};
use content_mapp_rs::indexer::Indexer;
use content_mapp_rs::web;

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

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let n = cli.n.unwrap_or(5);

    let dir = cli.dir.unwrap_or("./".into());

    let mut indexer = Indexer::new(dir, n)?;
    indexer.index_all_files()?;
    match cli.command {
        Commands::Print => {
            indexer.print_results()?;
            indexer.save_state()?;
        }
        Commands::Serve => {
            let state = Arc::new(indexer);
            let app = web::get_router(state);

            let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
            open::that_detached("http://localhost:3000/").unwrap();
            axum::serve(listener, app).await.unwrap();
        }
    };

    Ok(())
}
