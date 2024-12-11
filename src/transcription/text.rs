use crate::{FileCategory, FileType};
use crate::transcription::error::TranscriptionError;
use crate::transcription::error::TranscriptionError::UnsupportedFileType;

pub fn transcribe_text(file: FileType) -> Result<String, TranscriptionError> {
    match file.category() {
        FileCategory::Text => {

        }
        _ => return Err(TranscriptionError::UnsupportedExtension)
    }
    super::read_string_file(file)
}