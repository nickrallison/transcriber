use std::path::{absolute, PathBuf};
use std::process::Command;
use serde::{Deserialize, Serialize};
use tempfile::tempdir;
use crate::error::Error;

// insanely-fast-whisper --file-name "The size of your variables matters. [hwyRnHA54lI].webm"
pub(crate) fn whisper_local(filepath: PathBuf) -> Result<String, Error> {
    let temp_dir = tempdir()?;
    let cmd_path = which::which("insanely-fast-whisper")?;
    let filepath_abs = absolute(filepath)?;
    let mut cmd = Command::new(cmd_path);
    cmd
        .current_dir(&temp_dir)
        .arg("--file-name")
        .arg(filepath_abs)
        .status()?;
    let output_json: PathBuf = temp_dir.path().join("output.json");
    let output_string = std::fs::read_to_string(output_json)?;
    let result: WhisperLocalResponse = serde_json::from_str(&output_string)?;
    Ok(result.text)
}

// {
//     "speakers": [],
//     "chunks": [
//         {
//             "timestamp": [
//                 0.0,
//                 307.16
//             ],
//             "text": " you you you you you you you you you you you you you you Hey everyone."
//         },
//         ...
//         {
//             "timestamp": [
//                 2457.0,
//                 2460.02
//             ],
//             "text": " I will see you all later then."
//         }
//     ],
//     "text": " you you you you you you you you you you you you you you Hey everyone..."
// }

#[derive(Serialize, Deserialize)]
struct WhisperLocalResponse {
    speakers: Vec<String>,
    chunks: Vec<WhisperLocalChunk>,
    text: String,
}

#[derive(Serialize, Deserialize)]
struct WhisperLocalChunk {
    timestamp: (f32, f32),
    text: String,
}