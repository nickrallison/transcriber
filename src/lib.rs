//! Transcribes a string into its associated contents
//! - Handles files
//! - Handles links

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

mod parse;
mod error;
mod transform;

use std::path::PathBuf;

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
    contents: String,
    file_type: FileCategory
}

/// This is an actual file in the filesystem.
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct PathFile {
    path: PathBuf,
    file_type: FileCategory
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
    let input_as_file: FileType = transform::transform_input(input_type)?;
    todo!()
}