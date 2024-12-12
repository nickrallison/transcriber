use crate::{FileCategory, FileType, StringFile};
use crate::transcription::error::TranscriptionError;

pub fn transcribe_audio(file: FileType) -> Result<StringFile, TranscriptionError> {
    match file.category() {
        FileCategory::Audio => {
            todo!()
        }
        _ => return Err(TranscriptionError::UnsupportedExtension)
    }
    todo!()
}