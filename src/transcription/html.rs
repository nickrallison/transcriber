use crate::{FileCategory, FileType, StringFile};
use crate::error::Error;

pub fn transcribe_html(file: FileType) -> Result<StringFile, Error> {
    match file.category() {
        FileCategory::Html => {
            todo!()
        }
        _ => return Err(Error::UnsupportedExtension(file.category()))
    }
    todo!()
}