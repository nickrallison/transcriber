use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    // #[error("Input was not a valid link and not a file that exists")]
    // InvalidInput(String),

    #[error("Input Parsing Error: {0}")]
    Parse(#[from] crate::parse::error::ParseError),

    #[error("Input Transformation Error: {0}")]
    Transform(#[from] crate::transform::error::TransformError),

    #[error("Input Validation Error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Unknown File Type: {0}")]
    UnknownFileType(PathBuf),
}