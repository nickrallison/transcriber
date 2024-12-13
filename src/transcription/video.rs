use crate::{FileCategory, FileType, StringFile};
use crate::error::Error;

pub fn transcribe_video(file: FileType) -> Result<StringFile, Error> {
    match file.category() {
        FileCategory::Video => {
            todo!()
        }
        _ => return Err(Error::UnsupportedExtension(file.category()))
    }
    todo!()
}