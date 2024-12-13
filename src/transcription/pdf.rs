use std::ffi::{OsStr, OsString};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::tempdir;
use crate::{FileCategory, FileType, PathFile, StringFile};
use crate::transcription::error::TranscriptionError;

pub fn transcribe_pdf(file: FileType) -> Result<StringFile, TranscriptionError> {
    match file.category() {
        FileCategory::Pdf => {
            let path_file: PathFile = match file {
                FileType::StringFile(_) => return Err(TranscriptionError::PdfStringFile),
                FileType::PathFile(path_file) => path_file,
                FileType::BytesFile(bytes_file) => {
                    // turn into real file with temp path
                    let mut temp_file = tempfile::NamedTempFile::new()?;
                    temp_file.write_all(&bytes_file.bytes)?;
                    let path_res = PathFile::new(temp_file.path().to_path_buf());
                    match path_res {
                        Ok(path_file) => path_file,
                        Err(_) => return Err(TranscriptionError::PdfStringFile),
                    }
                }
            };
            println!("cwd: {:?}", std::env::current_dir()?);
            let contents = doclings(&path_file.path).unwrap();
            let filename = path_file.filename;
            let file_category = FileCategory::Text;
            Ok(StringFile::new(filename, contents, file_category))
        }
        _ => Err(TranscriptionError::UnsupportedExtension)
    }
}

fn doclings(from_pdf: &Path) -> Result<String, TranscriptionError> {
    let pdf_abs_path = from_pdf.canonicalize().unwrap();
    let tempdir = tempdir().unwrap();
    let cmd_path = which::which("docling").unwrap();
    let mut cmd = Command::new(cmd_path);
    cmd.current_dir(tempdir.path());
    cmd.arg(pdf_abs_path);
    cmd.status().unwrap();
    // get all child files
    let mut files = vec![];
    for entry in tempdir.path().read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        println!("Path: {:?}", path);
        if path.is_file() {
            files.push(path);
        }
    }
    if files.len() != 1 {
        return Err(TranscriptionError::PdfStringFile);
    }
    let read = std::fs::read(&files[0]).unwrap();
    let cow_string = String::from_utf8_lossy(&read);
    Ok(cow_string.parse().unwrap())
}

#[cfg(test)]
mod pdf_transcribe_tests {
    use rstest::rstest;
    use crate::parse::parse_input;
    use crate::transcription::pdf::transcribe_pdf;
    use crate::transform::transform_input;

    #[test]
    fn test_pdf_transcribe() {
        let filename = "tests/VeriCoq A Verilog to Coq converter for proof carrying hardware automation.pdf";
        let input_file = parse_input(filename).unwrap();
        let transformed = transform_input(input_file).unwrap();
        assert_eq!(transformed.len(), 1);
        let transformed = transformed[0].clone();
        let result = transcribe_pdf(transformed).unwrap();
        let expected = std::fs::read_to_string("tests/VeriCoq A Verilog to Coq converter for proof carrying hardware automation.md").unwrap();
        assert_eq!(result.contents, expected);
    }
}