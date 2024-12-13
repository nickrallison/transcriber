use std::path::PathBuf;
use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    // #[error("Input was not a valid link and not a file that exists")]
    // InvalidInput(String),

    #[error("Youtube Regex: {0}, failed to find {1} id in this url: {2}")]
    YoutubeRegex(Regex, String, String),

    #[error("Youtube link: {0} failed to parse, this is likely because the link doesn't start with one of the following: {1:?}")]
    YoutubeLinkParse(String, &'static [&'static str]),

    #[error("Input Validation Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Unknown File Type: {0}")]
    UnknownFileType(PathBuf),
}
