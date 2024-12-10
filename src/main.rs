mod file;
//
// use std::ffi::OsStr;
// use which::which;
// use url::Url;
// use std::process::{Command, Stdio};
// use tempfile::{tempfile, NamedTempFile, TempDir};
// use std::{env, fs};
// use std::fs::File;
// use std::io::Write;
// use std::path::{Path, PathBuf};
// use srtlib::{Subtitle, Subtitles};
// // use regex::Regex;
// use fancy_regex::{CaptureMatches, RegexBuilder};
// use regex::Regex;
//
// // Check if required programs are installed
// fn check_dependencies() -> Result<(), String> {
//     // let required_programs = ["wget", "yt-dlp", "pandoc", "ffmpeg", "whisper", "pdftotext"];
//     let required_programs = ["wget", "yt-dlp", "pandoc", "ffmpeg", "pdftotext"];
//     for program in required_programs.iter() {
//         if which(program).is_err() {
//             return Err(format!("{} is not installed.", program));
//         }
//     }
//     Ok(())
// }
//
// // Classify the input as file, YouTube URL, or other website
// enum InputType {
//     File(String),
//     Youtube,
//     Website,
// }
//
// fn classify_input(input: &str) -> Result<InputType, String> {
//     if input.starts_with("http") {
//         match Url::parse(input) {
//             Ok(url) => {
//                 if url.host_str() == Some("www.youtube.com") || url.host_str() == Some("youtube.com") {
//                     Ok(InputType::Youtube)
//                 } else {
//                     Ok(InputType::Website)
//                 }
//             },
//             Err(_) => Err("Invalid URL.".to_string()),
//         }
//     } else {
//         // Assume it's a file path
//         Ok(InputType::File(input.to_string()))
//     }
// }
//
// // Handle website content
// fn handle_website(url: &str) -> Result<String, String> {
//     // Download the webpage using wget
//     let output = Command::new("wget")
//         .arg("-q")
//         .arg("-O-")
//         .arg(url)
//         .output()
//         .map_err(|e| format!("Failed to download URL: {}", e))?;
//
//     // Convert HTML to text using pandoc
//     let mut child = Command::new("pandoc")
//         .arg("-")
//         .arg("--from=html")
//         .arg("--to=text")
//         .stdin(Stdio::piped())
//         .stdout(Stdio::piped())
//         .spawn()
//         .map_err(|e| format!("Failed to start pandoc: {}", e))?;
//
//     if let Some(mut stdin) = child.stdin.take() {
//         std::thread::scope(|s| {
//             s.spawn(|| {
//                 match stdin.write_all(&output.stdout) {
//                     Ok(_) => (),
//                     Err(e) => eprintln!("Failed to write to pandoc stdin: {}", e),
//                 }
//             });
//         });
//     }
//
//     let output = child.wait_with_output().map_err(|e| format!("Failed to read pandoc output: {}", e))?;
//     Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
// }
//
// fn handle_srt_sub(file_contents: String) -> Result<String, String> {
//     // strip first 3
//     let pattern = Regex::new(r"(?m)\d{2}:\d{2}:\d{2}\.\d{3} --> \d{2}:\d{2}:\d{2}\.\d{3} align:start position:\d+%").unwrap();
//     let inner_pattern = Regex::new(r"<.*?>").unwrap();
//     let ws_pattern = Regex::new(r"\s+").unwrap();
//     // remove all lines that don't match
//     let file_contents = pattern.replace_all(&file_contents, "");
//     let file_contents = inner_pattern.replace_all(&file_contents, "");
//     let file_contents = ws_pattern.replace_all(&file_contents, " ").to_string();
//     // join all matches into a single string
//     Ok(file_contents)
//
// }
//
// // Handle YouTube URL
// fn handle_youtube(url: &str) -> Result<String, String> {
//
//     // save cwd
//     // let cwd = env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;
//
//     // make tempdir and change to it
//     let tmp = TempDir::new().map_err(|e| format!("Failed to create temporary directory:  {}", e))?;
//     // std::env::set_current_dir(&tmp.path()).map_err(|e| format!("Failed to set current directory:  {}", e))?;
//
//     // Use yt-dlp to download transcript
//     let output = Command::new("yt-dlp")
//         .current_dir(&tmp)
//         .arg("--write-auto-sub")
//         .arg("--skip-download")
//         .arg("--sub-lang")
//         .arg("en")
//         .arg("--sub-format")
//         .arg("srt")
//         .arg(url)
//         .output()
//         .map_err(|e| format!("Failed to download YouTube transcript: {}", e))?;
//
//     if output.status.success() {
//         // Assuming the transcript is saved in a temporary file, read it
//         // let transcript_path = "transcript.vtt"; // yt-dlp default
//
//         // should be a single file in temp with ext .vtt
//         let mut files = tmp.path().read_dir().map_err(|e| format!("Failed to read temporary directory:   {}", e))?;
//
//         // assert 1 file
//         let mut files = files.filter(|f| f.is_ok()).map(|f| f.unwrap()).collect::<Vec<_>>();
//         if files.len() != 1 {
//             return Err(format!("Expected exactly one file in temporary directory, but found {}.", files.len()));
//         }
//
//         let transcript_path = files[0].path();
//         let transcript_path = transcript_path.to_str().ok_or("Failed to convert path to string".to_string())?;
//         let transcript = std::fs::read_to_string(transcript_path).map_err(|e| format!("Failed to read temporary file: {}", e))?;
//
//         let mut transcript = handle_srt_sub(transcript)?;
//         Ok(transcript)
//
//     } else {
//         Err("Failed to download YouTube transcript.".to_string())
//     }
// }
//
// // Handle file based on type
// fn handle_file(path: &str) -> Result<String, String> {
//     if path.ends_with(".html") || path.ends_with(".htm") {
//         handle_html_file(path)
//     } else if path.ends_with(".pdf") {
//         handle_pdf_file(path)
//     } else if path.ends_with(".mp4") || path.ends_with(".avi") {
//         handle_video_file(path)
//     } else {
//         Err(format!("Unsupported file type: {}", path))
//     }
// }
//
// // Handle HTML file
// fn handle_html_file(path: &str) -> Result<String, String> {
//     // Convert HTML to text using pandoc
//     let output = Command::new("pandoc")
//         .arg(path)
//         .arg("--from=html")
//         .arg("--to=text")
//         .output()
//         .map_err(|e| format!("Failed to convert HTML to text: {}", e))?;
//
//     Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
// }
//
// // Handle PDF file
// fn handle_pdf_file(path: &str) -> Result<String, String> {
//     // Use pdftotext or similar tool to extract text from PDF
//     let mut child = Command::new("pdftotext")
//         .arg(path)
//         .arg("-")
//         .stdout(Stdio::piped())
//         .spawn()
//         .map_err(|e| format!("Failed to start pdftotext: {}", e))?;
//
//     let output = child.wait_with_output().map_err(|e| format!("Failed to read pdftotext output: {}", e))?;
//     Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
// }
//
// // Handle video file
// fn handle_video_file(path: &str) -> Result<String, String> {
//
//     todo!("TODO: handle video files & Fix Whisper Transcription");
//
//     // Extract audio using ffmpeg
//     let temp_audio: NamedTempFile = NamedTempFile::new().map_err(|e| format!("Failed to create temporary audio file: {}", e))?;
//     let audio_path = temp_audio.into_temp_path();
//
//     Command::new("ffmpeg")
//         .arg("-i")
//         .arg(path)
//         .arg("-vn")
//         .arg("-acodec")
//         .arg("pcm_s16le")
//         .arg("-ar")
//         .arg("16000")
//         .arg("-ac")
//         .arg("1")
//         .arg(&audio_path)
//         .output()
//         .map_err(|e| format!("Failed to extract audio: {}", e))?;
//
//     // Transcribe audio using whisper
//     let output = Command::new("whisper")
//         .arg(audio_path)
//         .output()
//         .map_err(|e| format!("Failed to transcribe audio: {}", e))?;
//
//     Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
// }
//
// // Main function to handle the input
// fn main_handler(input: &str) -> Result<String, String> {
//     // check_dependencies()?;
//
//     let handled_file: String = match classify_input(input)? {
//         InputType::File(path) => handle_file(&input),
//         InputType::Youtube => handle_youtube(&input),
//         InputType::Website => handle_website(&input),
//     }?;
//
//     Ok(handled_file)
// }
//
// fn main() {
//     let input = "https://www.youtube.com/watch?v=dQw4w9WgXcQ"; // Example YouTube URL
//     match main_handler(input) {
//         Ok(content) => println!("{}", content),
//         Err(e) => eprintln!("Error: {}", e),
//     }
// }
//
// // tests
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//
//     const srt_file_cont: &str = r"WEBVTT
// Kind: captions
// Language: en
//
// 00:00:00.000 --> 00:00:18.790 align:start position:0%
//
// [Music]
//
// 00:00:18.790 --> 00:00:18.800 align:start position:0%
//
//
//
// 00:00:18.800 --> 00:00:21.790 align:start position:0%
//
// we're<00:00:19.039><c> no</c><00:00:19.279><c> strangers</c><00:00:19.800><c> to</c>
//
// 00:00:21.790 --> 00:00:21.800 align:start position:0%
// we're no strangers to
//
//
// 00:00:21.800 --> 00:00:26.029 align:start position:0%
// we're no strangers to
// love<00:00:22.800><c> you</c><00:00:23.000><c> know</c><00:00:23.320><c> the</c><00:00:23.519><c> rules</c><00:00:24.400><c> and</c><00:00:24.560><c> so</c><00:00:25.160><c> do</c>
//
// 00:00:26.029 --> 00:00:26.039 align:start position:0%
// love you know the rules and so do
//
//
// 00:00:26.039 --> 00:00:29.630 align:start position:0%
// love you know the rules and so do
// I<00:00:27.039><c> I</c><00:00:27.199><c> full</c><00:00:27.640><c> commitments</c><00:00:28.359><c> while</c><00:00:28.640><c> I'm</c><00:00:29.119><c> thinking</c>
//
// 00:00:29.630 --> 00:00:29.640 align:start position:0%
// I I full commitments while I'm thinking
//
//
// 00:00:29.640 --> 00:00:31.230 align:start position:0%
// I I full commitments while I'm thinking
// of
//
// 00:00:31.230 --> 00:00:31.240 align:start position:0%
// of
//
//
// 00:00:31.240 --> 00:00:35.510 align:start position:0%
// of
// you<00:00:31.480><c> wouldn't</c><00:00:32.040><c> get</c><00:00:32.279><c> this</c><00:00:32.559><c> from</c><00:00:33.200><c> any</c><00:00:33.680><c> other</c><00:00:34.440><c> guy</c>
//
// 00:00:35.510 --> 00:00:35.520 align:start position:0%
// you wouldn't get this from any other guy
//
//
// 00:00:35.520 --> 00:00:39.430 align:start position:0%
// you wouldn't get this from any other guy
// I<00:00:36.520><c> just</c><00:00:36.719><c> want</c><00:00:36.920><c> to</c><00:00:37.239><c> tell</c><00:00:37.520><c> you</c><00:00:37.800><c> how</c><00:00:38.160><c> I'm</c>
//
// 00:00:39.430 --> 00:00:39.440 align:start position:0%
// I just want to tell you how I'm
//
//
// 00:00:39.440 --> 00:00:43.350 align:start position:0%
// I just want to tell you how I'm
// feeling<00:00:40.440><c> got</c><00:00:40.600><c> to</c><00:00:41.000><c> make</c><00:00:41.360><c> you</c><00:00:42.360><c> understand</c><00:00:43.120><c> Never</c>
//
// 00:00:43.350 --> 00:00:43.360 align:start position:0%
// feeling got to make you understand Never
//
//
// 00:00:43.360 --> 00:00:46.189 align:start position:0%
// feeling got to make you understand Never
// Going<00:00:43.440><c> To</c><00:00:43.760><c> Give</c><00:00:44.120><c> You</c><00:00:44.520><c> Up</c><00:00:45.200><c> never</c><00:00:45.440><c> going</c><00:00:45.520><c> to</c><00:00:45.760><c> let</c>
//
// 00:00:46.189 --> 00:00:46.199 align:start position:0%
// Going To Give You Up never going to let
//
//
// 00:00:46.199 --> 00:00:49.709 align:start position:0%
// Going To Give You Up never going to let
// you<00:00:46.640><c> down</c><00:00:47.320><c> never</c><00:00:47.559><c> going</c><00:00:47.640><c> to</c><00:00:47.920><c> run</c><00:00:48.520><c> around</c><00:00:49.239><c> and</c>
//
// 00:00:49.709 --> 00:00:49.719 align:start position:0%
// you down never going to run around and
//
//
// 00:00:49.719 --> 00:00:53.630 align:start position:0%
// you down never going to run around and
// desert<00:00:50.559><c> you</c><00:00:51.559><c> never</c><00:00:51.760><c> going</c><00:00:51.879><c> to</c><00:00:52.120><c> make</c><00:00:52.440><c> you</c><00:00:52.879><c> cry</c>
//
// 00:00:53.630 --> 00:00:53.640 align:start position:0%
// desert you never going to make you cry
//
//
// 00:00:53.640 --> 00:00:56.110 align:start position:0%
// desert you never going to make you cry
// never<00:00:53.840><c> going</c><00:00:54.000><c> to</c><00:00:54.199><c> say</c><00:00:54.800><c> goodbye</c><00:00:55.800><c> never</c><00:00:56.039><c> going</c>
//
// 00:00:56.110 --> 00:00:56.120 align:start position:0%
// never going to say goodbye never going
//
//
// 00:00:56.120 --> 00:01:00.750 align:start position:0%
// never going to say goodbye never going
// to<00:00:56.320><c> tell</c><00:00:56.800><c> a</c><00:00:57.199><c> lie</c><00:00:58.160><c> and</c><00:00:58.440><c> hurt</c><00:00:59.000><c> you</c>
//
// 00:01:00.750 --> 00:01:00.760 align:start position:0%
// to tell a lie and hurt you
//
//
// 00:01:00.760 --> 00:01:03.950 align:start position:0%
// to tell a lie and hurt you
// we've<00:01:01.000><c> known</c><00:01:01.399><c> each</c><00:01:01.840><c> other</c><00:01:02.840><c> for</c><00:01:03.120><c> so</c>
//
// 00:01:03.950 --> 00:01:03.960 align:start position:0%
// we've known each other for so
//
//
// 00:01:03.960 --> 00:01:07.310 align:start position:0%
// we've known each other for so
// long<00:01:04.960><c> your</c><00:01:05.239><c> heart's</c><00:01:05.560><c> been</c><00:01:05.840><c> aching</c><00:01:06.439><c> but</c><00:01:07.040><c> your</c>
//
// 00:01:07.310 --> 00:01:07.320 align:start position:0%
// long your heart's been aching but your
//
//
// 00:01:07.320 --> 00:01:10.590 align:start position:0%
// long your heart's been aching but your
// to<00:01:07.560><c> sh</c><00:01:08.159><c> to</c><00:01:08.520><c> say</c><00:01:08.759><c> it</c><00:01:09.280><c> inside</c><00:01:09.840><c> we</c><00:01:10.080><c> both</c><00:01:10.320><c> know</c>
//
// 00:01:10.590 --> 00:01:10.600 align:start position:0%
// to sh to say it inside we both know
//
//
// 00:01:10.600 --> 00:01:12.510 align:start position:0%
// to sh to say it inside we both know
// what's<00:01:10.880><c> been</c><00:01:11.479><c> going</c>
//
// 00:01:12.510 --> 00:01:12.520 align:start position:0%
// what's been going
//
//
// 00:01:12.520 --> 00:01:16.630 align:start position:0%
// what's been going
// on<00:01:13.520><c> we</c><00:01:13.720><c> know</c><00:01:14.000><c> the</c><00:01:14.280><c> game</c><00:01:14.560><c> and</c><00:01:14.840><c> we're</c><00:01:15.640><c> going</c><00:01:15.759><c> to</c>
//
// 00:01:16.630 --> 00:01:16.640 align:start position:0%
// on we know the game and we're going to
//
//
// 00:01:16.640 --> 00:01:21.390 align:start position:0%
// on we know the game and we're going to
// playing<00:01:17.840><c> and</c><00:01:18.840><c> if</c><00:01:19.040><c> you</c><00:01:19.439><c> ask</c><00:01:19.799><c> me</c><00:01:20.119><c> how</c><00:01:20.360><c> I'm</c>
//
// 00:01:21.390 --> 00:01:21.400 align:start position:0%
// playing and if you ask me how I'm
//
//
// 00:01:21.400 --> 00:01:24.590 align:start position:0%
// playing and if you ask me how I'm
// feeling<00:01:22.400><c> don't</c><00:01:22.640><c> tell</c><00:01:22.920><c> me</c><00:01:23.159><c> you're</c><00:01:23.439><c> too</c><00:01:24.079><c> my</c><00:01:24.320><c> you</c>
//
// 00:01:24.590 --> 00:01:24.600 align:start position:0%
// feeling don't tell me you're too my you
//
//
// 00:01:24.600 --> 00:01:27.710 align:start position:0%
// feeling don't tell me you're too my you
// see<00:01:25.360><c> Never</c><00:01:25.600><c> Going</c><00:01:25.720><c> To</c><00:01:26.000><c> Give</c><00:01:26.400><c> You</c><00:01:26.799><c> Up</c><00:01:27.479><c> never</c>
//
// 00:01:27.710 --> 00:01:27.720 align:start position:0%
// see Never Going To Give You Up never
//
//
// 00:01:27.720 --> 00:01:30.789 align:start position:0%
// see Never Going To Give You Up never
// going<00:01:27.799><c> to</c><00:01:28.079><c> let</c><00:01:28.439><c> you</c><00:01:28.880><c> down</c><00:01:29.520><c> never</c><00:01:30.000><c> to</c><00:01:30.159><c> run</c>
//
// 00:01:30.789 --> 00:01:30.799 align:start position:0%
// going to let you down never to run
//
//
// 00:01:30.799 --> 00:01:34.389 align:start position:0%
// going to let you down never to run
// around<00:01:31.520><c> and</c><00:01:32.000><c> desert</c><00:01:32.799><c> you</c><00:01:33.799><c> never</c><00:01:34.040><c> going</c><00:01:34.159><c> to</c>
//
// 00:01:34.389 --> 00:01:34.399 align:start position:0%
// around and desert you never going to
//
//
// 00:01:34.399 --> 00:01:38.069 align:start position:0%
// around and desert you never going to
// make<00:01:34.720><c> you</c><00:01:35.119><c> cry</c><00:01:35.920><c> never</c><00:01:36.159><c> going</c><00:01:36.240><c> to</c><00:01:36.439><c> say</c><00:01:37.079><c> goodbye</c>
//
// 00:01:38.069 --> 00:01:38.079 align:start position:0%
// make you cry never going to say goodbye
//
//
// 00:01:38.079 --> 00:01:42.230 align:start position:0%
// make you cry never going to say goodbye
// never<00:01:38.280><c> going</c><00:01:38.399><c> to</c><00:01:38.600><c> tell</c><00:01:39.040><c> a</c><00:01:39.439><c> lie</c><00:01:40.399><c> and</c><00:01:40.720><c> hurt</c><00:01:41.280><c> you</c>
//
// 00:01:42.230 --> 00:01:42.240 align:start position:0%
// never going to tell a lie and hurt you
//
//
// 00:01:42.240 --> 00:01:44.670 align:start position:0%
// never going to tell a lie and hurt you
// never<00:01:42.479><c> going</c><00:01:42.600><c> to</c><00:01:42.880><c> give</c><00:01:43.240><c> you</c><00:01:43.640><c> up</c><00:01:44.360><c> never</c><00:01:44.600><c> going</c>
//
// 00:01:44.670 --> 00:01:44.680 align:start position:0%
// never going to give you up never going
//
//
// 00:01:44.680 --> 00:01:47.670 align:start position:0%
// never going to give you up never going
// to<00:01:44.960><c> let</c><00:01:45.280><c> you</c><00:01:45.680><c> down</c><00:01:46.479><c> never</c><00:01:46.719><c> going</c><00:01:46.840><c> to</c><00:01:47.079><c> run</c>
//
// 00:01:47.670 --> 00:01:47.680 align:start position:0%
// to let you down never going to run
//
//
// 00:01:47.680 --> 00:01:51.310 align:start position:0%
// to let you down never going to run
// around<00:01:48.439><c> and</c><00:01:48.920><c> desert</c><00:01:49.719><c> you</c><00:01:50.719><c> never</c><00:01:50.960><c> going</c><00:01:51.040><c> to</c>
//
// 00:01:51.310 --> 00:01:51.320 align:start position:0%
// around and desert you never going to
//
//
// 00:01:51.320 --> 00:01:55.190 align:start position:0%
// around and desert you never going to
// make<00:01:51.600><c> you</c><00:01:52.040><c> cry</c><00:01:52.799><c> never</c><00:01:53.040><c> going</c><00:01:53.119><c> to</c><00:01:53.280><c> sing</c><00:01:54.200><c> goodbye</c>
//
// 00:01:55.190 --> 00:01:55.200 align:start position:0%
// make you cry never going to sing goodbye
//
//
// 00:01:55.200 --> 00:01:58.830 align:start position:0%
// make you cry never going to sing goodbye
// going<00:01:55.320><c> to</c><00:01:55.520><c> tell</c><00:01:55.960><c> a</c><00:01:56.399><c> lie</c><00:01:57.360><c> and</c><00:01:57.600><c> hurt</c>
//
// 00:01:58.830 --> 00:01:58.840 align:start position:0%
// going to tell a lie and hurt
//
//
// 00:01:58.840 --> 00:02:01.950 align:start position:0%
// going to tell a lie and hurt
// you
//
// 00:02:01.950 --> 00:02:01.960 align:start position:0%
//
//
//
// 00:02:01.960 --> 00:02:05.230 align:start position:0%
//
// give
//
// 00:02:05.230 --> 00:02:05.240 align:start position:0%
//
//
//
// 00:02:05.240 --> 00:02:07.630 align:start position:0%
//
// you<00:02:06.240><c> give</c>
//
// 00:02:07.630 --> 00:02:07.640 align:start position:0%
// you give
//
//
// 00:02:07.640 --> 00:02:11.869 align:start position:0%
// you give
// you<00:02:08.640><c> going</c><00:02:08.759><c> to</c><00:02:09.000><c> give</c><00:02:09.479><c> going</c><00:02:09.560><c> to</c><00:02:09.800><c> give</c>
//
// 00:02:11.869 --> 00:02:11.879 align:start position:0%
// you going to give going to give
//
//
// 00:02:11.879 --> 00:02:15.830 align:start position:0%
// you going to give going to give
// you<00:02:12.879><c> going</c><00:02:13.000><c> to</c><00:02:13.200><c> give</c><00:02:13.680><c> going</c><00:02:13.800><c> to</c><00:02:14.040><c> give</c>
//
// 00:02:15.830 --> 00:02:15.840 align:start position:0%
// you going to give going to give
//
//
// 00:02:15.840 --> 00:02:20.030 align:start position:0%
// you going to give going to give
// you<00:02:16.840><c> we've</c><00:02:17.080><c> known</c><00:02:17.440><c> each</c><00:02:17.879><c> other</c><00:02:18.879><c> for</c><00:02:19.160><c> so</c>
//
// 00:02:20.030 --> 00:02:20.040 align:start position:0%
// you we've known each other for so
//
//
// 00:02:20.040 --> 00:02:23.350 align:start position:0%
// you we've known each other for so
// long<00:02:21.040><c> your</c><00:02:21.280><c> heart's</c><00:02:21.599><c> been</c><00:02:21.879><c> aching</c><00:02:22.480><c> but</c><00:02:23.120><c> you're</c>
//
// 00:02:23.350 --> 00:02:23.360 align:start position:0%
// long your heart's been aching but you're
//
//
// 00:02:23.360 --> 00:02:26.910 align:start position:0%
// long your heart's been aching but you're
// too<00:02:23.599><c> sh</c><00:02:24.239><c> to</c><00:02:24.560><c> say</c><00:02:25.400><c> inside</c><00:02:25.920><c> we</c><00:02:26.120><c> both</c><00:02:26.360><c> know</c><00:02:26.640><c> what's</c>
//
// 00:02:26.910 --> 00:02:26.920 align:start position:0%
// too sh to say inside we both know what's
//
//
// 00:02:26.920 --> 00:02:28.470 align:start position:0%
// too sh to say inside we both know what's
// been<00:02:27.560><c> going</c>
//
// 00:02:28.470 --> 00:02:28.480 align:start position:0%
// been going
//
//
// 00:02:28.480 --> 00:02:32.710 align:start position:0%
// been going
// on<00:02:29.480><c> we</c><00:02:30.080><c> the</c><00:02:30.280><c> game</c><00:02:30.599><c> and</c><00:02:30.920><c> we're</c><00:02:31.680><c> going</c><00:02:31.800><c> to</c><00:02:32.200><c> play</c>
//
// 00:02:32.710 --> 00:02:32.720 align:start position:0%
// on we the game and we're going to play
//
//
// 00:02:32.720 --> 00:02:37.750 align:start position:0%
// on we the game and we're going to play
// it<00:02:33.800><c> I</c><00:02:34.800><c> just</c><00:02:35.040><c> want</c><00:02:35.200><c> to</c><00:02:35.560><c> tell</c><00:02:35.800><c> you</c><00:02:36.160><c> how</c><00:02:36.440><c> I'm</c>
//
// 00:02:37.750 --> 00:02:37.760 align:start position:0%
// it I just want to tell you how I'm
//
//
// 00:02:37.760 --> 00:02:41.630 align:start position:0%
// it I just want to tell you how I'm
// feeling<00:02:38.760><c> got</c><00:02:38.920><c> to</c><00:02:39.319><c> make</c><00:02:39.640><c> you</c><00:02:40.640><c> understand</c><00:02:41.400><c> Never</c>
//
// 00:02:41.630 --> 00:02:41.640 align:start position:0%
// feeling got to make you understand Never
//
//
// 00:02:41.640 --> 00:02:44.470 align:start position:0%
// feeling got to make you understand Never
// Going<00:02:41.760><c> To</c><00:02:42.080><c> Give</c><00:02:42.440><c> You</c><00:02:42.879><c> Up</c><00:02:43.519><c> never</c><00:02:43.760><c> going</c><00:02:43.840><c> to</c><00:02:44.159><c> let</c>
//
// 00:02:44.470 --> 00:02:44.480 align:start position:0%
// Going To Give You Up never going to let
//
//
// 00:02:44.480 --> 00:02:48.070 align:start position:0%
// Going To Give You Up never going to let
// you<00:02:44.920><c> down</c><00:02:45.599><c> never</c><00:02:45.879><c> going</c><00:02:46.000><c> to</c><00:02:46.239><c> run</c><00:02:46.879><c> around</c><00:02:47.560><c> and</c>
//
// 00:02:48.070 --> 00:02:48.080 align:start position:0%
// you down never going to run around and
//
//
// 00:02:48.080 --> 00:02:51.949 align:start position:0%
// you down never going to run around and
// desert<00:02:48.879><c> you</c><00:02:49.879><c> never</c><00:02:50.080><c> going</c><00:02:50.200><c> to</c><00:02:50.480><c> make</c><00:02:50.800><c> you</c><00:02:51.200><c> cry</c>
//
// 00:02:51.949 --> 00:02:51.959 align:start position:0%
// desert you never going to make you cry
//
//
// 00:02:51.959 --> 00:02:54.430 align:start position:0%
// desert you never going to make you cry
// never<00:02:52.200><c> going</c><00:02:52.280><c> to</c><00:02:52.560><c> say</c><00:02:53.080><c> goodbye</c><00:02:54.080><c> never</c><00:02:54.360><c> going</c>
//
// 00:02:54.430 --> 00:02:54.440 align:start position:0%
// never going to say goodbye never going
//
//
// 00:02:54.440 --> 00:02:58.630 align:start position:0%
// never going to say goodbye never going
// to<00:02:54.680><c> tell</c><00:02:55.120><c> you</c><00:02:55.560><c> my</c><00:02:56.440><c> and</c><00:02:56.760><c> Hurt</c><00:02:57.319><c> You</c><00:02:58.280><c> Never</c><00:02:58.560><c> Going</c>
//
// 00:02:58.630 --> 00:02:58.640 align:start position:0%
// to tell you my and Hurt You Never Going
//
//
// 00:02:58.640 --> 00:03:00.390 align:start position:0%
// to tell you my and Hurt You Never Going
// To<00:02:58.920><c> Give</c><00:02:59.239><c> You</c><00:02:59.560><c> Up</c>
//
// 00:03:00.390 --> 00:03:00.400 align:start position:0%
// To Give You Up
//
//
// 00:03:00.400 --> 00:03:02.869 align:start position:0%
// To Give You Up
// never<00:03:00.640><c> going</c><00:03:00.760><c> to</c><00:03:01.040><c> let</c><00:03:01.360><c> you</c><00:03:01.840><c> down</c><00:03:02.560><c> never</c><00:03:02.760><c> going</c>
//
// 00:03:02.869 --> 00:03:02.879 align:start position:0%
// never going to let you down never going
//
//
// 00:03:02.879 --> 00:03:07.070 align:start position:0%
// never going to let you down never going
// to<00:03:03.120><c> run</c><00:03:03.760><c> around</c><00:03:04.480><c> and</c><00:03:04.959><c> desert</c><00:03:05.760><c> you</c><00:03:06.760><c> never</c><00:03:07.000><c> going</c>
//
// 00:03:07.070 --> 00:03:07.080 align:start position:0%
// to run around and desert you never going
//
//
// 00:03:07.080 --> 00:03:11.030 align:start position:0%
// to run around and desert you never going
// to<00:03:07.360><c> make</c><00:03:07.599><c> you</c><00:03:08.200><c> C</c><00:03:08.879><c> never</c><00:03:09.120><c> going</c><00:03:09.200><c> to</c><00:03:09.440><c> say</c><00:03:10.040><c> goodbye</c>
//
// 00:03:11.030 --> 00:03:11.040 align:start position:0%
// to make you C never going to say goodbye
//
//
// 00:03:11.040 --> 00:03:12.390 align:start position:0%
// to make you C never going to say goodbye
// never<00:03:11.239><c> going</c><00:03:11.319><c> to</c>
//
// 00:03:12.390 --> 00:03:12.400 align:start position:0%
// never going to
//
//
// 00:03:12.400 --> 00:03:16.630 align:start position:0%
// never going to
// tell<00:03:13.400><c> and</c><00:03:13.680><c> Hur</c><00:03:14.239><c> You</c><00:03:15.239><c> Never</c><00:03:15.440><c> Going</c><00:03:15.519><c> To</c><00:03:15.879><c> Give</c><00:03:16.239><c> You</c>
//
// 00:03:16.630 --> 00:03:16.640 align:start position:0%
// tell and Hur You Never Going To Give You
//
//
// 00:03:16.640 --> 00:03:19.670 align:start position:0%
// tell and Hur You Never Going To Give You
// Up<00:03:17.319><c> never</c><00:03:17.560><c> going</c><00:03:17.640><c> to</c><00:03:17.959><c> let</c><00:03:18.319><c> you</c><00:03:18.760><c> down</c><00:03:19.480><c> never</c>
//
// 00:03:19.670 --> 00:03:19.680 align:start position:0%
// Up never going to let you down never
//
//
// 00:03:19.680 --> 00:03:23.949 align:start position:0%
// Up never going to let you down never
// going<00:03:19.760><c> to</c><00:03:20.040><c> run</c><00:03:20.680><c> around</c><00:03:21.400><c> and</c><00:03:21.879><c> desert</c><00:03:22.680><c> you</c><00:03:23.680><c> never</c>
//
// 00:03:23.949 --> 00:03:23.959 align:start position:0%
// going to run around and desert you never
//
//
// 00:03:23.959 --> 00:03:27.020 align:start position:0%
// going to run around and desert you never
// going<00:03:24.040><c> to</c><00:03:24.360><c> make</c><00:03:25.040><c> you</c><00:03:26.040><c> going</c><00:03:26.120><c> to</c>
//
// 00:03:27.020 --> 00:03:27.030 align:start position:0%
// going to make you going to
//
//
// 00:03:27.030 --> 00:03:28.670 align:start position:0%
// going to make you going to
// [Music]
//
// 00:03:28.670 --> 00:03:28.680 align:start position:0%
// [Music]
//
//
// 00:03:28.680 --> 00:03:30.990 align:start position:0%
// [Music]
// goodbye
//
// 00:03:30.990 --> 00:03:31.000 align:start position:0%
// goodbye
//
//
// 00:03:31.000 --> 00:03:34.000 align:start position:0%
// goodbye
// and
// ";
//     #[test]
//     fn test_handle_srt() {
//         let transcript = handle_srt_sub(srt_file_cont.to_string());
//         assert!(transcript.is_ok());
//         let transcript = transcript.unwrap();
//         let expected = r"WEBVTT Kind: captions Language: en [Music] we're no strangers to we're no strangers to we're no strangers to love you know the rules and so do love you know the rules and so do love you know the rules and so do I I full commitments while I'm thinking I I full commitments while I'm thinking I I full commitments while I'm thinking of of of you wouldn't get this from any other guy you wouldn't get this from any other guy you wouldn't get this from any other guy I just want to tell you how I'm I just want to tell you how I'm I just want to tell you how I'm feeling got to make you understand Never feeling got to make you understand Never feeling got to make you understand Never Going To Give You Up never going to let Going To Give You Up never going to let Going To Give You Up never going to let you down never going to run around and you down never going to run around and you down never going to run around and desert you never going to make you cry desert you never going to make you cry desert you never going to make you cry never going to say goodbye never going never going to say goodbye never going never going to say goodbye never going to tell a lie and hurt you to tell a lie and hurt you to tell a lie and hurt you we've known each other for so we've known each other for so we've known each other for so long your heart's been aching but your long your heart's been aching but your long your heart's been aching but your to sh to say it inside we both know to sh to say it inside we both know to sh to say it inside we both know what's been going what's been going what's been going on we know the game and we're going to on we know the game and we're going to on we know the game and we're going to playing and if you ask me how I'm playing and if you ask me how I'm playing and if you ask me how I'm feeling don't tell me you're too my you feeling don't tell me you're too my you feeling don't tell me you're too my you see Never Going To Give You Up never see Never Going To Give You Up never see Never Going To Give You Up never going to let you down never to run going to let you down never to run going to let you down never to run around and desert you never going to around and desert you never going to around and desert you never going to make you cry never going to say goodbye make you cry never going to say goodbye make you cry never going to say goodbye never going to tell a lie and hurt you never going to tell a lie and hurt you never going to tell a lie and hurt you never going to give you up never going never going to give you up never going never going to give you up never going to let you down never going to run to let you down never going to run to let you down never going to run around and desert you never going to around and desert you never going to around and desert you never going to make you cry never going to sing goodbye make you cry never going to sing goodbye make you cry never going to sing goodbye going to tell a lie and hurt going to tell a lie and hurt going to tell a lie and hurt you give you give you give you give you going to give going to give you going to give going to give you going to give going to give you going to give going to give you going to give going to give you going to give going to give you we've known each other for so you we've known each other for so you we've known each other for so long your heart's been aching but you're long your heart's been aching but you're long your heart's been aching but you're too sh to say inside we both know what's too sh to say inside we both know what's too sh to say inside we both know what's been going been going been going on we the game and we're going to play on we the game and we're going to play on we the game and we're going to play it I just want to tell you how I'm it I just want to tell you how I'm it I just want to tell you how I'm feeling got to make you understand Never feeling got to make you understand Never feeling got to make you understand Never Going To Give You Up never going to let Going To Give You Up never going to let Going To Give You Up never going to let you down never going to run around and you down never going to run around and you down never going to run around and desert you never going to make you cry desert you never going to make you cry desert you never going to make you cry never going to say goodbye never going never going to say goodbye never going never going to say goodbye never going to tell you my and Hurt You Never Going to tell you my and Hurt You Never Going to tell you my and Hurt You Never Going To Give You Up To Give You Up To Give You Up never going to let you down never going never going to let you down never going never going to let you down never going to run around and desert you never going to run around and desert you never going to run around and desert you never going to make you C never going to say goodbye to make you C never going to say goodbye to make you C never going to say goodbye never going to never going to never going to tell and Hur You Never Going To Give You tell and Hur You Never Going To Give You tell and Hur You Never Going To Give You Up never going to let you down never Up never going to let you down never Up never going to let you down never going to run around and desert you never going to run around and desert you never going to run around and desert you never going to make you going to going to make you going to going to make you going to [Music] [Music] [Music] goodbye goodbye goodbye and ";
//         assert_eq!(transcript, expected);
//     }
// }