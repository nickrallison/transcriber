use crate::{FileCategory, FileType, StringFile};
use crate::error::Error;

pub fn transcribe_audio(file: FileType) -> Result<StringFile, Error> {
    match file.category() {
        FileCategory::Audio => {
            todo!()
        }
        _ => return Err(Error::UnsupportedExtension(file.category()))
    }
    todo!()
}