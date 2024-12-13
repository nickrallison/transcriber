use std::ffi::OsStr;
use std::path::Path;
use crate::error::Error;
use crate::FileCategory;

pub fn get_file_type(file_path: &Path) -> Result<FileCategory, Error> {
    let category = match file_path.extension().and_then(OsStr::to_str) {
        Some("mp4") => FileCategory::Video,
        Some("mkv") => FileCategory::Video,
        Some("avi") => FileCategory::Video,

        Some("mp3") => FileCategory::Audio,
        Some("flac") => FileCategory::Audio,
        Some("m4a") => FileCategory::Audio,

        Some("pdf") => FileCategory::Pdf,

        Some("md") => FileCategory::Text,
        Some("txt") => FileCategory::Text,

        Some("html") => FileCategory::Html,

        Some("srt") => FileCategory::Srt,
        Some("vtt") => FileCategory::Srt,

        _ => return Err(Error::UnknownFileType(file_path.to_path_buf())),
    };
    Ok(category)
}
