use crate::{FileCategory, StringFile};
use crate::error::Error;

const BAD_SRT_WORDS: [&str; 2] = ["-->","</c>"];
fn contains_bad_srt_words(srt_line: &str) -> bool {
    for word in BAD_SRT_WORDS.iter() {
        if srt_line.contains(word) {
            return true;
        }
    }
    false
}

fn clean_srt(srt: &str) -> String {
    let mut result: Vec<String> = vec![];
    for line in srt.lines() {
        let line = line.trim();
        let prev_line = result.last();
        if !contains_bad_srt_words(line) && !line.is_empty() {
            if let Some(prev_line) = prev_line {
                if line != prev_line {
                    result.push(line.to_string());
                }
            } else {
                result.push(line.to_string());
            }
        }
    }
    let mut result = result.join(" ");
    let illegal_pattern = "WEBVTT Kind: captions Language: en";
    let len_pattern = illegal_pattern.len();
    if result.starts_with("WEBVTT Kind: captions Language: en") {
        result = result[len_pattern + 1..].to_string();
    }
    result.trim().to_string()
}

pub fn transcribe_srt(file: crate::FileType) -> Result<StringFile, Error> {
    match file.category() {
        FileCategory::Srt => {
            let filename = crate::get_filename(file.filename().as_ref())
                .as_os_str()
                .to_os_string();
            let file_contents: String = super::read_string_file(file)?;
            let transcribe_result = clean_srt(&file_contents);
            Ok(StringFile::new(filename, transcribe_result, FileCategory::Text))
        }
        _ => Err(Error::UnsupportedExtension(file.category()))
    }
}