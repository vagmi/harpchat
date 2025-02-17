use std::env;

use axum::{routing::get, Router};

use crate::error::Result;

pub async fn start_server() -> Result<()> {
    let app = Router::new().route("/", get(|| async { "Hello, world!" }));
    let port = env::var("PORT").unwrap_or_else(|_| "8888".to_string());
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Starting server on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
