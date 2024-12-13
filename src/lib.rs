//! Transcribes a string into its associated contents
//! - Handles files
//! - Handles links
//! TODO: Add debug_asserts
//! TODO: Add Documentation, including @param, requires and promises
//! TODO: Add more tests
//! TODO: Serde for tests


#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

mod parse;
mod error;
mod transform;
mod transcription;
mod util;

use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::ffi::{OsStr, OsString};
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

use crate::error::Error;

/// These are what we can classify any one parse into
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub enum InputType {
    Website(WebsiteType),
    File(FileType)
}

/// Either a file in the filesystem or a string and a file type.
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub enum FileType {
    StringFile(StringFile),
    PathFile(PathFile),
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

    fn filename(&self) -> &OsStr {
        match self {
            FileType::StringFile(string_file) => string_file.filename(),
            FileType::PathFile(path_file) => path_file.filename(),
        }
    }
}

/// This is a string that is pretending to be a file.
/// No use making a file if not necessary.
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct StringFile {
    file_name: OsString,
    contents: String,
    file_type: FileCategory
}

impl StringFile {
    fn new(file_name: OsString, contents: String, file_type: FileCategory) -> Self {
        StringFile {
            file_name,
            contents,
            file_type
        }
    }

    fn category(&self) -> FileCategory {
        (self.file_type).clone()
    }

    fn filename(&self) -> &OsStr {
        &self.file_name
    }
}

/// This is an actual file in the filesystem.
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct PathFile {
    path: PathBuf,
    file_type: FileCategory,
    filename: OsString
}

impl PathFile {
    fn category(&self) -> FileCategory {
        (self.file_type).clone()
    }

    fn filename(&self) -> &OsStr {
        &self.path.file_name().expect("Path Should have filename")
    }
}

// /// This is an actual file in the filesystem.
// #[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
// pub struct BytesFile {
//     file_name: OsString,
//     bytes: Vec<u8>,
//     file_type: FileCategory
// }
// 
// impl BytesFile {
//     fn category(&self) -> FileCategory {
//         (self.file_type).clone()
//     }
// 
//     fn filename(&self) -> &OsStr {
//         &self.file_name
//     }
// }



impl PathFile {
    fn new(path: PathBuf) -> Result<PathFile, Error> {
        let category = crate::util::get_file_type(&path)?;
        let filename: OsString = crate::get_filename(&path).as_os_str().to_os_string();
        Ok(PathFile {
            path,
            file_type: category,
            filename,
        })
    }
}


/// The type of a file.
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub enum FileCategory {
    Audio,
    Video,
    Html,
    Pdf,
    Text,
    Srt
}

impl Display for FileCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            FileCategory::Audio => "Audio",
            FileCategory::Video => "Video",
            FileCategory::Html => "Html",
            FileCategory::Pdf => "Pdf",
            FileCategory::Text => "Text",
            FileCategory::Srt => "Srt",
        };
        write!(f, "{}", res)
    }
}

/// This is what we can classify any one website into
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub enum WebsiteType {
    Youtube(parse::youtube::YoutubeType),
    Article(String)
}

/// Transcribes some input into a string.
///
/// @param input The input to transcribe.
///
pub fn transcribe(input: &str) -> Result<Vec<Result<StringFile, Error>>, crate::error::Error> {
    let input_type: InputType = parse::parse_input(input)?;
    let files: Vec<FileType> = transform::transform_input(input_type)?;
    let transcriptions: Vec<Result<StringFile, Error>> = files.into_par_iter().map(|file| transcription::transcribe_file(file)).collect();
    Ok(transcriptions)
}

fn get_filename(path: &Path) -> &Path {
    let filename: &Path = Path::new(path.file_name().expect("Path should have filename"));
    let filename = Path::file_stem(filename).expect("path should have had filename");
    Path::new(filename)
}

#[cfg(test)]
mod top_tests {
    use rstest::rstest;
    use crate::transcription::transcribe_file;
    use super::*;

    #[test]
    fn test_transcribe() {
        let input = "https://www.youtube.com/watch?v=gsbRweN-FMk";
        let transcriptions = transcribe(input);
        assert!(transcriptions.is_ok());
        let transcriptions = transcriptions.unwrap();
        for transcription in transcriptions {
            println!("{:?}", transcription);
        }
    }

    #[test]
    fn test_helper() {
        let input = "https://www.youtube.com/watch?v=7yrK_9PderQ&list=WL&index=3&pp=gAQBiAQB";
        let input_type = parse::parse_input(&input).unwrap();
        let files = transform::transform_input(input_type).unwrap();
        let json = serde_json::to_string(&files).unwrap();
        println!("input: {:?}", json);
        let results: Vec<StringFile> = files
            .into_iter()
            .map(|file| transcribe_file(file).unwrap())
            .collect();
        let json = serde_json::to_string(&results).unwrap();
        println!("res: {:?}", json);
        // panic!();
    }
}