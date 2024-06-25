use axum_embed::ServeEmbed;
use rust_embed::RustEmbed;
use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};
use tokio::sync::Mutex;

use crate::indexer::Indexer;

#[derive(RustEmbed, Clone)]
#[folder = "static/"]
struct Assets;

pub type AppState = Arc<Mutex<Indexer>>;

pub async fn file_connections_get(State(indexer): State<AppState>) -> impl IntoResponse {
    let mut indexer = indexer.lock().await;
    indexer.index_modified_files().unwrap();
    indexer.save_state().await.unwrap();
    Json(indexer.get_file_sim_json().unwrap())
}

pub fn get_router(state: AppState) -> Router {
    let serve_assets = ServeEmbed::<Assets>::new();

    Router::new()
        // .route("/", get(root_get))
        .nest_service("/", serve_assets)
        .route("/file_connections", get(file_connections_get))
        .with_state(state)
}
