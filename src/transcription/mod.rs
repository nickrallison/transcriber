use crate::FileType;
use crate::transcription::error::TranscriptionError;

mod srt;
pub(crate) mod error;

pub fn transcribe(file: FileType) -> Result<String, TranscriptionError> {
    todo!()
}