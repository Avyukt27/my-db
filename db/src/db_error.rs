use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("I/O Error Occured: {0}")]
    Io(#[from] std::io::Error),
    #[error("Encountered error in parsing: {0}")]
    ParseError(String),
    #[error("Key not found: {0}")]
    KeyNotFound(String),
}
