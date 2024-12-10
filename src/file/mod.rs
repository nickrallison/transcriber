pub(crate) mod input;
mod error;

use std::path::PathBuf;

/// These are what we can classify any one input into
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
    Youtube(input::youtube::YoutubeType),
    Article(String)
}
