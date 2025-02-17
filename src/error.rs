use thiserror::Error;

pub type Result<T> = std::result::Result<T, HarpError>;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum HarpError {
    #[error("An error occured: {0}")]
    GenericError(String),
    #[error("Error starting server on port 8888: {0}")]
    IoError(#[from] std::io::Error),
}
