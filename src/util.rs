use std::ffi::OsStr;
use std::path::Path;
use regex::Regex;
use crate::error::{Error, UtilError};
use crate::FileCategory;

lazy_static::lazy_static! {
    static ref VALID_CHAR: Regex = Regex::new(r"^[a-zA-Z0-9\s\.\,]$").unwrap();
}
pub fn clean_filename(string: String) -> String {
    let utf8_string = deunicode::deunicode(&string);
    let mut cleaned_string = String::new();
    for c in utf8_string.chars() {
        if VALID_CHAR.is_match(&c.to_string()) {
            cleaned_string.push(c);
        }
    }
    cleaned_string
}


pub(crate) fn get_file_type(file_path: &Path) -> Result<FileCategory, UtilError> {
    let category = match file_path.extension().and_then(OsStr::to_str) {
        Some("mp4") => FileCategory::Video,
        Some("mkv") => FileCategory::Video,
        Some("avi") => FileCategory::Video,
        Some("webm") => FileCategory::Video,

        Some("mp3") => FileCategory::Audio,
        Some("flac") => FileCategory::Audio,
        Some("m4a") => FileCategory::Audio,

        Some("pdf") => FileCategory::Pdf,

        Some("md") => FileCategory::Text,
        Some("txt") => FileCategory::Text,

        Some("html") => FileCategory::Html,

        Some("srt") => FileCategory::Srt,
        Some("vtt") => FileCategory::Srt,

        _ => return Err(UtilError::UnknownFileType(file_path.to_path_buf())),
    };
    Ok(category)
}

#[cfg(test)]
mod util_tests{
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("video.mp4", Ok(FileCategory::Video))]
    #[case("movie.mkv", Ok(FileCategory::Video))]
    #[case("clip.avi", Ok(FileCategory::Video))]
    #[case(r"C:\Users\Nick\Downloads\Why should you learn Type Theory？ [QRrcwahx-3s].webm", Ok(FileCategory::Video))]
    #[case("song.mp3", Ok(FileCategory::Audio))]
    #[case("track.flac", Ok(FileCategory::Audio))]
    #[case("file.m4a", Ok(FileCategory::Audio))]
    #[case("document.pdf", Ok(FileCategory::Pdf))]
    #[case("readme.md", Ok(FileCategory::Text))]
    #[case("notes.txt", Ok(FileCategory::Text))]
    #[case("page.html", Ok(FileCategory::Html))]
    #[case("subtitle.srt", Ok(FileCategory::Srt))]
    #[case("captions.vtt", Ok(FileCategory::Srt))]
    #[case("unknown.bin", Err(UtilError::UnknownFileType(Path::new("unknown.bin").to_path_buf())))]
    fn test_get_file_type(#[case] input: &str, #[case] expected: Result<FileCategory, UtilError>) {
        let file_path = Path::new(input);
        let result = get_file_type(file_path);
        assert_eq!(result, expected);
    }
}