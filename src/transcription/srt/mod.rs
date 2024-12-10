

const bad_srt_words: [&str; 2] = ["-->","</c>"];
fn contains_bad_srt_words(srt_line: &str) -> bool {
    for word in bad_srt_words.iter() {
        if srt_line.contains(word) {
            return true;
        }
    }
    return false;
}

pub fn clean_srt(srt: &str) -> String {
    srt.lines().filter(|line| !contains_bad_srt_words(line)).collect::<Vec<&str>>().join("\n")
}