//! Transcribes a string into its associated contents
//! - Handles files
//! - Handles links
//! TODO: Add debug_asserts
//! TODO: Add Documentation, including @param, requires and promises
//! TODO: Add more tests


#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

mod parse;
mod error;
mod transform;
mod transcription;
mod util;

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use crate::error::Error;

/// These are what we can classify any one parse into
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum InputType {
    Website(WebsiteType),
    File(FileType)
}

/// Either a file in the filesystem or a string and a file type.
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum FileType {
    StringFile(StringFile),
    PathFile(PathFile)
}

impl FileType {

    fn new_path(path: PathBuf) -> Result<FileType, Error> {
        Ok(FileType::PathFile(PathFile::new(path)?))
    }

    /// Grabs the file type from a file path
    fn category(&self) -> FileCategory {
        match self {
            Self::StringFile(str_file) => {
                str_file.file_type.clone()
            },
            Self::PathFile(path_file) => {
                path_file.file_type.clone()
            },
        }
    }

}

/// This is a string that is pretending to be a file.
/// No use making a file if not necessary.
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct StringFile {
    file_name: OsString,
    contents: String,
    file_type: FileCategory
}

/// This is an actual file in the filesystem.
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct PathFile {
    path: PathBuf,
    file_type: FileCategory
}

impl PathFile {
    fn new(path: PathBuf) -> Result<PathFile, Error> {
        let category = crate::util::get_file_type(&path)?;
        Ok(PathFile {
            path,
            file_type: category,
        })
    }
}


/// The type of a file.
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum FileCategory {
    Audio,
    Video,
    Html,
    Pdf,
    Text,
    Srt
}

/// This is what we can classify any one website into
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum WebsiteType {
    Youtube(parse::youtube::YoutubeType),
    Article(String)
}

/// Transcribes some input into a string.
///
/// @param input The input to transcribe.
///
pub fn transcribe(input: &str) -> Result<String, crate::error::Error> {
    let input_type: InputType = parse::parse_input(input)?;
    let input_as_file: Vec<FileType> = transform::transform_input(input_type)?;
    todo!()
}