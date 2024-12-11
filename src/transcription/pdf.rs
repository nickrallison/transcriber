use crate::{FileCategory, FileType};
use crate::transcription::error::TranscriptionError;

pub fn transcribe_pdf(file: FileType) -> Result<String, TranscriptionError> {
    match file.category() {
        FileCategory::Pdf => {
            todo!()
        }
        _ => return Err(TranscriptionError::UnsupportedExtension)
    }
    todo!()
}