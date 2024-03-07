use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use axum::{
    extract::State,
    http::Response,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
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

async fn root_get() -> impl IntoResponse {
    let html = tokio::fs::read_to_string("static/index.html")
        .await
        .expect("The file static/index.html must exist");
    Html(html)
}
async fn indexjs_get() -> impl IntoResponse {
    let js = tokio::fs::read_to_string("static/index.js")
        .await
        .expect("The file static/index.js must exist");
    Response::builder()
        .header("content-type", "application/javascript;charset=utf-8")
        .body(js)
        .unwrap()
}

async fn file_connections_get(State(indexer): State<AppState>) -> impl IntoResponse {
    Json(indexer.get_file_sim_json().unwrap())
}

type AppState = Arc<Indexer>;

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
        }
        Commands::Serve => {
            let state = Arc::new(indexer);
            let app = Router::new()
                .route("/", get(root_get))
                .route("/index.js", get(indexjs_get))
                .route("/file_connections", get(file_connections_get))
                .with_state(state);

            let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
            open::that_detached("http://localhost:3000/").unwrap();
            axum::serve(listener, app).await.unwrap();
        }
    };
    Ok(())
}
