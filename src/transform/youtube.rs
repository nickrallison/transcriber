//! Youtube Management for Transformation Lib
//! TODO: Add Documentation to crate::transform::youtube
//! TODO: Add Tests to crate::transform::youtube
//! TODO: Add Debug Asserts to crate::transform::youtube

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tempfile::tempdir;
use crate::error::Error;
use crate::{FileCategory, FileType, StringFile};
use crate::parse::youtube::YoutubeType;
use crate::transform::error::TransformError;

const YTDLP_LINK_WINDOWS: &str = "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe";
const YTDLP_LINK_LINUX: &str = "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp";
const YTDLP_LINK_DARWIN: &str = "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_macos";


fn ytdlp_binary() -> Result<PathBuf, TransformError> {
    let which_bin = which::which("yt-dlp");
    if which_bin.is_ok() {
        return Ok(which_bin.expect("Which should have found yt-dlp"))
    }
    // let download = ytdlp_download();
    // if download.is_err() {
    //     return Err(TransformError::Ytdlp("yt-dlp both failed to be found on system and failed to install".to_string()));
    // }
    // Ok(download.expect("yt-dlp should have been downloaded"))
    Err(TransformError::Ytdlp("yt-dlp both failed to be found on system and failed to install".to_string()))
}

fn ytdlp_download() -> Result<PathBuf, TransformError>  {
    let tempdir = tempdir()?;
    let os = std::env::consts::OS;
    let download_link = match os {
        "windows" => reqwest::blocking::get(YTDLP_LINK_WINDOWS),
        "linux" => reqwest::blocking::get(YTDLP_LINK_LINUX),
        "darwin" => reqwest::blocking::get(YTDLP_LINK_DARWIN),
        _ => return Err(TransformError::UnsupportedOS(os.to_string())),
    };
    if download_link.is_err() {
        return Err(TransformError::Ytdlp("yt-dlp download failed".to_string()));
    }
    let response = download_link.expect("yt-dlp download link should have been found");

    let filename = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .expect("should have had filename");

    let filepath = tempdir.path().join(filename);
    let mut dest = File::create(&filepath)?;

    let content =  response.bytes()?;
    dest.write_all(&content)?;
    Ok(filepath)
}

/// Transforms YouTube enum into associated file(s)
pub fn transform_youtube(youtube_type: YoutubeType) -> Result<Vec<crate::FileType>, crate::transform::error::TransformError> {
    // require yt-dlp to be installed
    which::which("yt-dlp")?;

    let files: Vec<crate::FileType> = match youtube_type {
        YoutubeType::Video(vid_id) => transform_youtube_video(vid_id)?,
        YoutubeType::Playlist(playlist_id) => transform_youtube_playlist(playlist_id)?,
        YoutubeType::Channel(channel_id) => transform_youtube_channel(channel_id)?,
        YoutubeType::ChannelAt(channel_id) => transform_youtube_channel_at(channel_id)?,
    };

    Ok(files)
}

/// Grabs a YouTube video transcript with yt-dlp
fn transform_youtube_video(vid_id: String) -> Result<Vec<crate::FileType>, crate::transform::error::TransformError> {
    debug_assert!(which::which("yt-dlp").is_ok());

    // Get a temp directory to run the command from
    let temp_dir = tempfile::tempdir()?;
    let yt_dlp_exe = ytdlp_binary()?;
    let mut cmd = std::process::Command::new(yt_dlp_exe);

    // setting the run directory of the command
    cmd.current_dir(temp_dir.path());

    cmd.arg("--skip-download");
    cmd.arg("--write-auto-sub");
    cmd.arg("--sub-format");
    // format subs as srt
    cmd.arg("srt");
    cmd.arg("--sub-lang");
    // get english subs
    cmd.arg("en");

    cmd.arg(vid_id);

    // exectute the command
    let out = cmd.output()?;

    if !out.status.success() {
        return Err(crate::transform::error::TransformError::CommandError(String::from_utf8(out.stderr)?));
    }

    let srt_files = glob::glob(&format!("{}/*", temp_dir.path().to_str().unwrap())).unwrap();

    let mut files: Vec<FileType> = Vec::new();
    for file in srt_files {
        let file_name = file.expect("Files in glob should exist");
        let contents = std::fs::read_to_string(&file_name).expect("Files in glob should exist");
        let string_file = StringFile {
            file_name: file_name.file_name().expect("Should have filename").to_os_string(),
            contents,
            file_type: FileCategory::Srt,
        };
        files.push(FileType::StringFile(string_file));
    }
    Ok(files)
}

/// Grabs the transcript of each video from a playlist with yt-dlp
fn transform_youtube_playlist(playlist_id: String) -> Result<Vec<crate::FileType>, crate::transform::error::TransformError> {
    debug_assert!(which::which("yt-dlp").is_ok());

    // Get a temp directory to run the command from
    let temp_dir = tempfile::tempdir()?;
    let yt_dlp_exe = ytdlp_binary()?;
    let mut cmd = std::process::Command::new(yt_dlp_exe);

    // setting the run directory of the command
    cmd.current_dir(temp_dir.path());

    cmd.arg("--skip-download");
    cmd.arg("--write-auto-sub");
    cmd.arg("--sub-format");
    // format subs as srt
    cmd.arg("srt");
    cmd.arg("--sub-lang");
    // get english subs
    cmd.arg("en");

    let playlist_id_arg = format!("https://www.youtube.com/playlist?list={}", playlist_id);

    cmd.arg(playlist_id_arg);

    // exectute the command
    let out = cmd.output()?;

    if !out.status.success() {
        return Err(crate::transform::error::TransformError::CommandError(String::from_utf8(out.stderr)?));
    }

    let srt_files = glob::glob(&format!("{}/*", temp_dir.path().to_str().unwrap())).unwrap();

    let mut files: Vec<FileType> = Vec::new();
    for file in srt_files {
        let file_name = file.expect("Files in glob should exist");
        let contents = std::fs::read_to_string(&file_name).expect("Files in glob should exist");
        let string_file = StringFile {
            file_name: file_name.file_name().expect("Should have filename").to_os_string(),
            contents,
            file_type: FileCategory::Srt,
        };
        files.push(FileType::StringFile(string_file));
    }
    Ok(files)
}

/// Grabs the transcript of each video from a channel with yt-dlp
fn transform_youtube_channel(channel_id: String) -> Result<Vec<crate::FileType>, crate::transform::error::TransformError> {
    debug_assert!(which::which("yt-dlp").is_ok());

    // Get a temp directory to run the command from
    let temp_dir = tempfile::tempdir()?;
    let yt_dlp_exe = ytdlp_binary()?;
    let mut cmd = std::process::Command::new(yt_dlp_exe);

    // setting the run directory of the command
    cmd.current_dir(temp_dir.path());

    cmd.arg("--skip-download");
    cmd.arg("--write-auto-sub");
    cmd.arg("--sub-format");
    // format subs as srt
    cmd.arg("srt");
    cmd.arg("--sub-lang");
    // get english subs
    cmd.arg("en");

    let channel_id_arg = format!("https://www.youtube.com/user/{}", channel_id);

    cmd.arg(channel_id_arg);

    // exectute the command
    let out = cmd.output()?;

    if !out.status.success() {
        return Err(crate::transform::error::TransformError::CommandError(String::from_utf8(out.stderr)?));
    }

    let srt_files = glob::glob(&format!("{}/*", temp_dir.path().to_str().unwrap())).unwrap();

    let mut files: Vec<FileType> = Vec::new();
    for file in srt_files {
        let file_name = file.expect("Files in glob should exist");
        let contents = std::fs::read_to_string(&file_name).expect("Files in glob should exist");
        let string_file = StringFile {
            file_name: file_name.file_name().expect("Should have filename").to_os_string(),
            contents,
            file_type: FileCategory::Srt,
        };
        files.push(FileType::StringFile(string_file));
    }
    Ok(files)
}

/// Grabs the transcript of each video from a channel with yt-dlp
fn transform_youtube_channel_at(channel_id: String) -> Result<Vec<crate::FileType>, crate::transform::error::TransformError> {
    debug_assert!(which::which("yt-dlp").is_ok());

    // Get a temp directory to run the command from
    let temp_dir = tempfile::tempdir()?;
    let yt_dlp_exe = ytdlp_binary()?;
    let mut cmd = std::process::Command::new(yt_dlp_exe);

    // setting the run directory of the command
    cmd.current_dir(temp_dir.path());

    cmd.arg("--skip-download");
    cmd.arg("--write-auto-sub");
    cmd.arg("--sub-format");
    // format subs as srt
    cmd.arg("srt");
    cmd.arg("--sub-lang");
    // get english subs
    cmd.arg("en");

    let channel_id_arg = format!("https://www.youtube.com/@{}", channel_id);

    cmd.arg(channel_id_arg);

    // exectute the command
    let out = cmd.output()?;

    if !out.status.success() {
        return Err(crate::transform::error::TransformError::CommandError(String::from_utf8(out.stderr)?));
    }

    let srt_files = glob::glob(&format!("{}/*", temp_dir.path().to_str().unwrap())).unwrap();

    let mut files: Vec<FileType> = Vec::new();
    for file in srt_files {
        let file_name = file.expect("Files in glob should exist");
        let contents = std::fs::read_to_string(&file_name).expect("Files in glob should exist");
        let string_file = StringFile {
            file_name: file_name.file_name().expect("Should have filename").to_os_string(),
            contents,
            file_type: FileCategory::Srt,
        };
        files.push(FileType::StringFile(string_file));
    }
    Ok(files)
}

#[cfg(test)]
mod youtube_transform_tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("https://www.youtube.com/watch?v=dQw4w9WgXcQ")]
    fn test_transform_youtube_video(#[case] url: &str) {
        let youtube_type = crate::parse::youtube::parse_youtube(url).expect(format!("Failed to parse {}", url).as_str());
        let result = transform_youtube(youtube_type).expect(format!("Failed to transform {}", url).as_str());
        assert_eq!(result.len(), 1);
        let vid_transcript = result.first().expect("Expected a single video");
        match vid_transcript {
            crate::FileType::StringFile(string_file) => {
                let file_type = string_file.file_type.clone();
                assert_eq!(file_type, crate::FileCategory::Srt);
                let contents = &string_file.contents;
                // println!("{}", contents);
                assert!(contents.starts_with("WEBVTT\nKind: captions\nLanguage: en\n\n00:00:00.000 -->"));
            }
            _ => panic!("Expected a PathFile")
        }

    }
}