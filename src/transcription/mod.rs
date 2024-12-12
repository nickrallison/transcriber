use crate::{FileCategory, FileType, StringFile};
use crate::transcription::audio::transcribe_audio;
use crate::transcription::error::TranscriptionError;
use crate::transcription::error::TranscriptionError::UnsupportedFileType;
use crate::transcription::html::transcribe_html;
use crate::transcription::pdf::transcribe_pdf;
use crate::transcription::srt::transcribe_srt;
use crate::transcription::text::transcribe_text;
use crate::transcription::video::transcribe_video;

pub(crate) mod error;
mod audio;
mod video;
mod html;
mod pdf;
mod srt;
mod text;

pub fn transcribe(file: FileType) -> Result<StringFile, TranscriptionError> {
    match file.category() {
        FileCategory::Audio => transcribe_audio(file),
        FileCategory::Video => transcribe_video(file),
        FileCategory::Html => transcribe_html(file),
        FileCategory::Pdf => transcribe_pdf(file),
        FileCategory::Text => transcribe_text(file),
        FileCategory::Srt => transcribe_srt(file),
    }
}

fn read_string_file(file_type: FileType) -> Result<String, TranscriptionError> {
    match file_type {
        FileType::StringFile(string_file) => Ok(string_file.contents),
        FileType::PathFile(path_file) => Ok(std::fs::read_to_string(path_file.path)?),
        FileType::BytesFile(_) => Err(UnsupportedFileType)
    }
}