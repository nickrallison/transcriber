//! Errors for Transformation Lib
//! TODO: Add Documentation to crate::transform::error
//! TODO: Add Tests to crate::transform::error
//! TODO: Add Debug Asserts to crate::transform::error
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransformError {

    #[error("Invalid input: {0}")]
    UrlScheme(String),

    #[error("Filename not found for: {0}")]
    Filename(String),

    #[error("Command Failed: {0}")]
    CommandError(String),

    #[error("Ytdlp Error: {0}")]
    Ytdlp(String),

    #[error("Unsupported Operating System: {0}")]
    UnsupportedOS(String),

    #[error("Unsupported File Type: {0}")]
    FileCategory(String),

    #[error("{0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),


    #[error("Io Error While Reading html response: {0}")]
    Io(#[from] std::io::Error),

    #[error("Web page did not return a valid response: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Web page did not return a valid response: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Input Transformation Error: {0}")]
    WhichError(#[from] which::Error),

    #[error("{0}")]
    Infallible(#[from] std::convert::Infallible),


}