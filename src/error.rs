//! Possible errors on the library

use thiserror::Error;

/// Standard result from the library.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Not enough data: {0}")]
    NotEnoughData(String),
    #[error("Unknown constituent: {0}")]
    UnknownConstituent(String),
    #[error("Invalida input: {0}")]
    InvalidInput(String),
    #[error("Input/Output error: {0}")]
    Io(#[from] std::io::Error),
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    #[error("JSON error: {0}:")]
    Json(#[from] serde_json::Error),
}
