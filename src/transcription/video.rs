use crate::{FileCategory, FileType};
use crate::transcription::error::TranscriptionError;

pub fn transcribe_video(file: FileType) -> Result<String, TranscriptionError> {
    match file.category() {
        FileCategory::Video => {
            todo!()
        }
        _ => return Err(TranscriptionError::UnsupportedExtension)
    }
    todo!()
}