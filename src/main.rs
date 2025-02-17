mod error;
mod state;
mod web;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    web::start_server().await.unwrap();
}
