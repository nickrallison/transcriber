
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TranscriptionError {
    // #[error("Input was not a valid link and not a file that exists")]
    // InvalidInput(String),

    // #[error("Input Parsing Error: {0}")]
    // Parse(#[from] crate::parse::error::ParseError),

}