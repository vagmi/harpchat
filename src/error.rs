use thiserror::Error;

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
}
