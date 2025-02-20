mod db;
mod error;
mod state;
mod model;
mod web;
mod ai;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let db_pool = db::setup_db().await.unwrap();
    let state = state::AppState { db_pool };
    web::start_server(state).await.unwrap();
}
