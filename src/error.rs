use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

use crate::model::Message;

pub type Result<T> = std::result::Result<T, HarpError>;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum HarpError {
    #[error("Environment variable not found: {0}")]
    VarError(#[from] std::env::VarError),
    #[error("An error occured: {0}")]
    GenericError(String),
    #[error("Error starting server on port 8888: {0}")]
    IoError(#[from] std::io::Error),
    #[error("DB Layer Error: {0}")]
    DbError(#[from] sqlx::error::Error),
    #[error("Error parsing JSON: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Error sending message: {0}")]
    SendError(#[from] tokio::sync::mpsc::error::SendError<Message>),
    #[error("Error processing with AI: {0}")]
    GenAIError(#[from] genai::Error),
}

impl IntoResponse for HarpError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
