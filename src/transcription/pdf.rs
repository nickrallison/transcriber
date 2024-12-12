use crate::{FileCategory, FileType, StringFile};
use crate::transcription::error::TranscriptionError;

pub fn transcribe_pdf(file: FileType) -> Result<StringFile, TranscriptionError> {
    match file.category() {
        FileCategory::Pdf => {
            todo!()
        }
        _ => return Err(TranscriptionError::UnsupportedExtension)
    }
    todo!()
}