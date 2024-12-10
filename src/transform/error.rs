
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransformError {

    #[error("Invalid input: {0}")]
    UrlScheme(String),

    #[error("Io Error While Reading html response: {0}")]
    Io(#[from] std::io::Error),

    #[error("Web page did not return a valid response: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Web page did not return a valid response: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Input Transformation Error: {0}")]
    WhichError(#[from] which::Error),

}