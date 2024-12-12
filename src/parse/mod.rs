//! Parsing
//! TODO: Add Documentation to crate::parse
//! TODO: Add Tests to crate::parse
//! TODO: Add Debug Asserts to crate::parse

pub(crate) mod youtube;
pub(crate) mod error;

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use crate::{FileCategory, FileType, InputType, PathFile, WebsiteType};
use crate::error::Error;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref YOUTUBE_REGEX: Regex = Regex::new(r"(?:https?://)?(?:www\.)?(?:youtube|youtu|youtube-nocookie)\.(?:com|be)").unwrap();
}

pub fn parse_input(input: &str) -> Result<InputType, crate::error::Error> {
    if input.contains("http") {
        return Ok(InputType::Website(parse_website(input)?));
    }
    Ok(InputType::File(parse_file(input)?))
}

fn get_extension(path: &Path) -> Option<FileCategory> {
    let file_type: FileCategory = match path.extension().and_then(OsStr::to_str) {
        Some("mp4") => FileCategory::Video,
        Some("mkv") => FileCategory::Video,
        Some("avi") => FileCategory::Video,

        Some("mp3") => FileCategory::Audio,
        Some("flac") => FileCategory::Audio,
        Some("m4a") => FileCategory::Audio,

        Some("pdf") => FileCategory::Pdf,

        Some("md") => FileCategory::Text,
        Some("txt") => FileCategory::Text,

        Some("html") => FileCategory::Html,

        Some("srt") => FileCategory::Srt,
        Some("vtt") => FileCategory::Srt,

        _ => return None,
    };
    Some(file_type)
}

fn parse_file(input: &str) -> Result<FileType, Error> {
    Ok(FileType::PathFile(PathFile::new(PathBuf::from(input.to_string()))?))
}

fn parse_website(input: &str) -> Result<WebsiteType, crate::error::Error> {

    if YOUTUBE_REGEX.is_match(input) {
        Ok(WebsiteType::Youtube(youtube::parse_youtube(input)?))
    }
    else {
        Ok(WebsiteType::Article(input.to_string()))
    }
}


#[cfg(test)]
mod test_input_parse {
    use rstest::rstest;
    use super::*;
    use crate::FileCategory;
 
    #[test]
    fn test_parse_youtube_video() {
        let input = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
        let result = parse_website(input);
        match result {
            Ok(youtube) => {
                assert_eq!(youtube, WebsiteType::Youtube(youtube::YoutubeType::Video("dQw4w9WgXcQ".to_string())));
            }
            Err(_) => {
                panic!("parse youtube video failed")
            }
        }
    }

    #[test]
    fn test_parse_article() {
        let input = "https://zipcpu.com/tutorial/";
        let result = parse_website(input);
        match result  {
            Ok(article) => {
                assert_eq!(article, WebsiteType::Article("https://zipcpu.com/tutorial/".to_string()));
             }
            Err(_)  => {
                panic!("parse article failed")
             }
         }
    }
    #[rstest]
    #[case("file.mp3", FileCategory::Audio)]
    #[case("/path/to/file.mp3", FileCategory::Audio)]
    #[case("C:\\Users\\Nick\\file.mp4", FileCategory::Video)]
    #[case("file.pdf", FileCategory::Pdf)]
    #[case("file.md", FileCategory::Text)]
    #[case("file.srt", FileCategory::Srt)]
    #[case("file.vtt", FileCategory::Srt)]
    fn test_parse_files(#[case] input: &str, #[case] category: FileCategory) {
        let result = parse_input(input);
        let input_enum = result.expect("Shoud be a file");
        let file = match input_enum {
            InputType::File(file) => file,
            _   => panic!("Should be a file")
        };
        assert_eq!(file.category(), category);
    }





}