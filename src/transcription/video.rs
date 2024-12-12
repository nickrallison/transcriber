use crate::{FileCategory, FileType, StringFile};
use crate::transcription::error::TranscriptionError;

pub fn transcribe_video(file: FileType) -> Result<StringFile, TranscriptionError> {
    match file.category() {
        FileCategory::Video => {
            todo!()
        }
        _ => return Err(TranscriptionError::UnsupportedExtension)
    }
    todo!()
}