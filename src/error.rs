use std::convert::Infallible;
use std::ffi::OsString;
use std::path::PathBuf;
use regex::Regex;
use thiserror::Error;
use crate::FileCategory;

#[derive(Error, Debug)]
pub enum Error {
    // #[error("Input was not a valid link and not a file that exists")]
    // InvalidInput(String),

    #[error("Youtube Regex: {0}, failed to find {1} id in this url: {2}")]
    YoutubeRegex(Regex, String, String),

    #[error("Youtube link: {0} failed to parse, this is likely because the link doesn't start with one of the following: {1:?}")]
    YoutubeLinkParse(String, &'static [&'static str]),

    #[error("Url: {0} is missing a filename and cannot be downloaded")]
    MissingFileName(String),

    #[error("yd-dlp is not found on the system, see this link: https://github.com/yt-dlp/yt-dlp?tab=readme-ov-file#installation")]
    YtdlpNotFound,

    #[error("yd-dlp failed with the following error: {0}")]
    CommandError(String),

    #[error("Unsupported file type: {0}")]
    UnsupportedExtension(FileCategory),

    #[error("The {0} File: {1:?} cannot be a StringFile")]
    InvalidFileTypeTranscribe(FileCategory, OsString),

    #[error("Too many files found: {0:?}")]
    TooManyFilesFromPDF(Vec<PathBuf>, String),

    #[error("doclings is not found on the system, see this link: https://ds4sd.github.io/docling/installation/")]
    CantFindDoclings,

    #[error("{0}")]
    Util(#[from] UtilError),

    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("{0}")]
    Infallable(#[from] Infallible),

    #[error("{0}")]
    CmdNotFound(#[from] which::Error),

    #[error("{0}")]
    SerdeDeserializable(#[from] serde_json::Error),

}

#[derive(Error, Debug, PartialEq)]
pub enum UtilError {
    #[error("Invalid extension: {0}")]
    UnknownFileType(PathBuf),
}
