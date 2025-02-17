use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum HarpError {
    #[error("An error occured: {0}")]
    GenericError(String),
}
