use std::ffi::OsStr;
use which::which;
use url::Url;
use std::process::{Command, Stdio};
use tempfile::{tempfile, NamedTempFile};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

// Check if required programs are installed
fn check_dependencies() -> Result<(), String> {
    let required_programs = ["wget", "yt-dlp", "pandoc", "ffmpeg", "whisper", "pdftotext"];
    for program in required_programs.iter() {
        if which(program).is_err() {
            return Err(format!("{} is not installed.", program));
        }
    }
    Ok(())
}

// Classify the input as file, YouTube URL, or other website
enum InputType {
    File(String),
    Youtube,
    Website,
}

fn classify_input(input: &str) -> Result<InputType, String> {
    if input.starts_with("http") {
        match Url::parse(input) {
            Ok(url) => {
                if url.host_str() == Some("www.youtube.com") || url.host_str() == Some("youtube.com") {
                    Ok(InputType::Youtube)
                } else {
                    Ok(InputType::Website)
                }
            },
            Err(_) => Err("Invalid URL.".to_string()),
        }
    } else {
        // Assume it's a file path
        Ok(InputType::File(input.to_string()))
    }
}

// Handle website content
fn handle_website(url: &str) -> Result<String, String> {
    // Download the webpage using wget
    let output = Command::new("wget")
        .arg("-q")
        .arg("-O-")
        .arg(url)
        .output()
        .map_err(|e| format!("Failed to download URL: {}", e))?;

    // Convert HTML to text using pandoc
    let mut child = Command::new("pandoc")
        .arg("-")
        .arg("--from=html")
        .arg("--to=text")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start pandoc: {}", e))?;

    if let Some(mut stdin) = child.stdin.take() {
        std::thread::scope(|s| {
            s.spawn(|| {
                match stdin.write_all(&output.stdout) {
                    Ok(_) => (),
                    Err(e) => eprintln!("Failed to write to pandoc stdin: {}", e),
                }
            });
        });
    }

    let output = child.wait_with_output().map_err(|e| format!("Failed to read pandoc output: {}", e))?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

// Handle YouTube URL
fn handle_youtube(url: &str) -> Result<String, String> {
    // Use yt-dlp to download transcript
    let output = Command::new("yt-dlp")
        .arg("--write-auto-sub")
        .arg("--skip-download")
        .arg("--sub-lang")
        .arg("en")
        .arg(url)
        .output()
        .map_err(|e| format!("Failed to download YouTube transcript: {}", e))?;

    if output.status.success() {
        // Assuming the transcript is saved in a temporary file, read it
        let transcript_path = "transcript.vtt"; // yt-dlp default
        match fs::read_to_string(transcript_path) {
            Ok(content) => Ok(content),
            Err(e) => Err(format!("Failed to read transcript: {}", e)),
        }
    } else {
        Err("Failed to download YouTube transcript.".to_string())
    }
}

// Handle file based on type
fn handle_file(path: &str) -> Result<String, String> {
    if path.ends_with(".html") || path.ends_with(".htm") {
        handle_html_file(path)
    } else if path.ends_with(".pdf") {
        handle_pdf_file(path)
    } else if path.ends_with(".mp4") || path.ends_with(".avi") {
        handle_video_file(path)
    } else {
        Err(format!("Unsupported file type: {}", path))
    }
}

// Handle HTML file
fn handle_html_file(path: &str) -> Result<String, String> {
    // Convert HTML to text using pandoc
    let output = Command::new("pandoc")
        .arg(path)
        .arg("--from=html")
        .arg("--to=text")
        .output()
        .map_err(|e| format!("Failed to convert HTML to text: {}", e))?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

// Handle PDF file
fn handle_pdf_file(path: &str) -> Result<String, String> {
    // Use pdftotext or similar tool to extract text from PDF
    let mut child = Command::new("pdftotext")
        .arg(path)
        .arg("-")
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start pdftotext: {}", e))?;

    let output = child.wait_with_output().map_err(|e| format!("Failed to read pdftotext output: {}", e))?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

// Handle video file
fn handle_video_file(path: &str) -> Result<String, String> {
    // Extract audio using ffmpeg
    let temp_audio: NamedTempFile = NamedTempFile::new().map_err(|e| format!("Failed to create temporary audio file: {}", e))?;
    let audio_path = temp_audio.into_temp_path();

    Command::new("ffmpeg")
        .arg("-i")
        .arg(path)
        .arg("-vn")
        .arg("-acodec")
        .arg("pcm_s16le")
        .arg("-ar")
        .arg("16000")
        .arg("-ac")
        .arg("1")
        .arg(&audio_path)
        .output()
        .map_err(|e| format!("Failed to extract audio: {}", e))?;

    // Transcribe audio using whisper
    let output = Command::new("whisper")
        .arg(audio_path)
        .output()
        .map_err(|e| format!("Failed to transcribe audio: {}", e))?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

// Main function to handle the input
fn main_handler(input: &str) -> Result<String, String> {
    check_dependencies()?;

    let handled_file: String = match classify_input(input)? {
        InputType::File(path) => handle_file(&input),
        InputType::Youtube => handle_youtube(&input),
        InputType::Website => handle_website(&input),
    }?;

    Ok(handled_file)
}

fn main() {
    let input = "https://www.youtube.com/watch?v=dQw4w9WgXcQ"; // Example YouTube URL
    match main_handler(input) {
        Ok(content) => println!("{}", content),
        Err(e) => eprintln!("Error: {}", e),
    }
}