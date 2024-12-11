use crate::{FileCategory, FileType};
use crate::transcription::error::TranscriptionError;

pub fn transcribe_audio(file: FileType) -> Result<String, TranscriptionError> {
    match file.category() {
        FileCategory::Audio => {
            todo!()
        }
        _ => return Err(TranscriptionError::UnsupportedExtension)
    }
    todo!()
}