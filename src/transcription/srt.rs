use crate::FileType;
use crate::transcription::error::TranscriptionError::UnsupportedFileType;

const bad_srt_words: [&str; 2] = ["-->","</c>"];
fn contains_bad_srt_words(srt_line: &str) -> bool {
    for word in bad_srt_words.iter() {
        if srt_line.contains(word) {
            return true;
        }
    }
    return false;
}

fn clean_srt(srt: &str) -> String {
    srt.lines().filter(|line| !contains_bad_srt_words(line)).collect::<Vec<&str>>().join("\n")
}

pub fn transcribe_srt(file: crate::FileType) -> Result<String, crate::transcription::error::TranscriptionError> {
    match file.category() {
        crate::FileCategory::Srt => {

        }
        _ => return Err(crate::transcription::error::TranscriptionError::UnsupportedExtension)
    }
    let file_contents: String = super::read_string_file(file)?;
    let transcribe_result = clean_srt(&*file_contents);
    Ok(transcribe_result)
}