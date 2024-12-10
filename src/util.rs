use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use rand::prelude::IteratorRandom;
use tempfile::tempdir;
use crate::error::Error;
use crate::FileCategory;

pub fn get_file_type(file_path: &Path) -> Result<FileCategory, Error> {
    let category = match file_path.extension().and_then(OsStr::to_str) {
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

        _ => return Err(Error::UnknownFileType(file_path.to_path_buf())),
    };
    Ok(category)
}

// #[derive(Debug)]
// pub struct TempDir {
//     path: PathBuf,
// }
// 
// impl TempDir {
//     pub fn new() -> Result<Self, Error> {
//         let mut temp_dir = tempdir()?;
//         println!("{:?}", temp_dir);
//         let char_range = 'a'..='z';
//         let mut random_string = String::new();
//         for _ in 0..10 {
//             let random_char = (char_range.clone()).choose(&mut rand::rng()).unwrap();
//             random_string.push(random_char);
//         }
//         
//         let mut temp_dir_path = temp_dir.path().to_path_buf();
//         temp_dir_path.push(random_string);
//         //mkdir
//         // println!("{:?}", temp_dir_path);
//         std::fs::create_dir(&temp_dir_path)?;
//         Ok(TempDir {
//             path: temp_dir_path,
//         })
//     }
//     pub fn clean_up(self) {
//         std::fs::remove_dir_all(&self.path).unwrap();
//     }
//     pub fn path(&self) -> &Path {
//         &self.path
//     }
// }
// 
// impl Display for TempDir {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let s = format!("{:?}", self.path);
//         write!(f, "{}", s)
//     }
// }