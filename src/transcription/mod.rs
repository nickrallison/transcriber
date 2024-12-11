use crate::{FileCategory, FileType};
use crate::transcription::audio::transcribe_audio;
use crate::transcription::error::TranscriptionError;
use crate::transcription::error::TranscriptionError::UnsupportedFileType;
use crate::transcription::text::transcribe_text;

pub(crate) mod error;
mod audio;
mod video;
mod html;
mod pdf;
mod srt;
mod text;

pub fn transcribe(file: FileType) -> Result<String, TranscriptionError> {
    match file.category() {
        FileCategory::Audio => transcribe_audio(file),
        FileCategory::Video => transcribe_audio(file),
        FileCategory::Html => transcribe_audio(file),
        FileCategory::Pdf => transcribe_audio(file),
        FileCategory::Text => transcribe_text(file),
        FileCategory::Srt => transcribe_audio(file),
    }
}

fn read_string_file(file_type: FileType) -> Result<String, TranscriptionError> {
    match file_type {
        FileType::StringFile(string_file) => Ok(string_file.contents),
        FileType::PathFile(path_file) => Ok(std::fs::read_to_string(path_file.path)?),
        FileType::BytesFile(_) => Err(UnsupportedFileType)
    }
}