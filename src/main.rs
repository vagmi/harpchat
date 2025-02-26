use std::sync::{Arc, Mutex};

use indexmap::IndexMap;

mod db;
mod error;
mod state;
mod model;
mod web;
mod ai;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let tx_map = Arc::new(Mutex::new(IndexMap::new()));
    let db_pool = db::setup_db().await.unwrap();
    let state = state::AppState { tx_map, db_pool };
    web::start_server(state).await.unwrap();
}
