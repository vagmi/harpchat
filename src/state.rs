use std::sync::{Arc, Mutex};

use indexmap::IndexMap;
use tower_sessions::session::Id;

use crate::model::Message;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AppState {
    pub tx_map: Arc<Mutex<IndexMap<Id, tokio::sync::mpsc::UnboundedSender<Message>>>>,
    pub db_pool: sqlx::PgPool,
}
