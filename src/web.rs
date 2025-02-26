use std::env;

mod routes;
mod views;

use crate::{error::Result, state::AppState};
use axum::Router;
use tower_sessions::{MemoryStore, SessionManagerLayer};


pub async fn start_server(state: AppState) -> Result<()> {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_always_save(true)
        .with_name("harp_sid");
    let app = Router::new()
        .merge(routes::setup_view_router())
        .with_state(state)
        .layer(session_layer);
    let port = env::var("PORT").unwrap_or_else(|_| "8888".to_string());
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Starting server on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
