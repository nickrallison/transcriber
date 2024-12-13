mod whisper_local;

use std::path::{absolute, PathBuf};
use std::process::Command;
use tempfile::tempdir;
use crate::{FileCategory, FileType, StringFile};
use crate::error::Error;

// 
pub fn transcribe_av(file: FileType) -> Result<StringFile, Error> {
    match file.category() {
        FileCategory::Video => (),
        FileCategory::Audio => (),
        _ => return Err(Error::UnsupportedExtension(file.category()))
    }
    match file {
        FileType::StringFile(string_file) => Err(Error::InvalidFileTypeTranscribe(FileCategory::Audio, string_file.file_name)),
        FileType::PathFile(path_file) => Ok(StringFile::new(path_file.filename, whisper_local::whisper_local(path_file.path)?, FileCategory::Text))
    }   
}
