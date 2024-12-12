use crate::{FileCategory, FileType, StringFile};
use crate::transcription::error::TranscriptionError;

pub fn transcribe_html(file: FileType) -> Result<StringFile, TranscriptionError> {
    match file.category() {
        FileCategory::Html => {
            todo!()
        }
        _ => return Err(TranscriptionError::UnsupportedExtension)
    }
    todo!()
}