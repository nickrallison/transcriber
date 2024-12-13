use crate::{FileCategory, FileType, StringFile};
use crate::error::Error;

pub fn transcribe_text(file: FileType) -> Result<StringFile, Error> {
    match file.category() {
        FileCategory::Text => Ok(StringFile::new(file.filename().to_os_string(), super::read_string_file(file)?, FileCategory::Text)),
        _ => Err(Error::UnsupportedExtension(file.category()))
    }
}