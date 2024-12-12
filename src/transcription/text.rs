use crate::{FileCategory, FileType, StringFile};
use crate::transcription::error::TranscriptionError;
use crate::transcription::error::TranscriptionError::UnsupportedFileType;

pub fn transcribe_text(file: FileType) -> Result<StringFile, TranscriptionError> {
    match file.category() {
        FileCategory::Text => {

        }
        _ => return Err(TranscriptionError::UnsupportedExtension)
    }
    Ok(StringFile::new(file.filename().to_os_string(), super::read_string_file(file)?, FileCategory::Text))
}