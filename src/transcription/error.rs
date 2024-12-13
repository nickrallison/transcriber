use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TranscriptionError {
    // #[error("Input was not a valid link and not a file that exists")]
    // InvalidInput(String),

    // #[error("Input Parsing Error: {0}")]
    // Parse(#[from] crate::parse::error::ParseError),

    #[error("Unsupported Extension Error")]
    UnsupportedExtension,

    #[error("Unsupported File Type")]
    UnsupportedFileType,
    
    #[error("{0}")]
    Io(#[from] std::io::Error)
}