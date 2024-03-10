use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};
use tower_http::services::{ServeDir, ServeFile};

use crate::indexer::Indexer;

pub async fn file_connections_get(State(indexer): State<AppState>) -> impl IntoResponse {
    Json(indexer.get_file_sim_json().unwrap())
}

pub type AppState = Arc<Indexer>;

pub fn get_router(state: AppState) -> Router {
    let serve_dir =
        ServeDir::new("./static").not_found_service(ServeFile::new("./static/index.html"));

    Router::new()
        // .route("/", get(root_get))
        .route("/file_connections", get(file_connections_get))
        .fallback_service(serve_dir)
        .with_state(state)
}
