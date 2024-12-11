use crate::{FileCategory, FileType};
use crate::transcription::error::TranscriptionError;

pub fn transcribe_html(file: FileType) -> Result<String, TranscriptionError> {
    match file.category() {
        FileCategory::Html => {
            todo!()
        }
        _ => return Err(TranscriptionError::UnsupportedExtension)
    }
    todo!()
}