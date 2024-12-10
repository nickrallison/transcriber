pub(crate) mod input;
mod error;

use std::path::PathBuf;


/// These are what we can classify any one input into
pub enum InputType {
    Website(WebsiteType),
    File(FileType)
}

/// Either a file in the filesystem or a string and a file type.
pub enum FileType {
    StringFile(StringFile),
    PathFile(PathFile)
}

/// This is a string that is pretending to be a file.
/// No use making a file if not necessary.
pub struct StringFile {
    contents: String,
    file_type: FileCategory
}

/// This is an actual file in the filesystem.
pub struct PathFile {
    path: PathBuf,
    file_type: FileCategory
}


/// The type of a file.
pub enum FileCategory {
    Audio,
    Video,
    Html,
    Text,
    Srt
}

/// This is what we can classify any one website into
pub enum WebsiteType {
    Youtube(input::youtube::YoutubeType),
    Article(String)
}
