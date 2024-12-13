use std::io::Write;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;
use crate::{FileCategory, FileType, PathFile, StringFile};
use crate::error::Error;

pub fn transcribe_pdf(file: FileType) -> Result<StringFile, Error> {
    match file.category() {
        FileCategory::Pdf => {
            let path_file: PathFile = match file {
                FileType::StringFile(string_file) => return Err(Error::PdfStringFile(string_file.file_name)),
                FileType::PathFile(path_file) => path_file,
            };
            println!("cwd: {:?}", std::env::current_dir()?);
            let contents = doclings(&path_file.path)?;
            let filename = path_file.filename;
            let file_category = FileCategory::Text;
            Ok(StringFile::new(filename, contents, file_category))
        }
        _ => Err(Error::UnsupportedExtension(file.category()))
    }
}

fn doclings(from_pdf: &Path) -> Result<String, Error> {
    let pdf_abs_path = from_pdf.canonicalize()?;
    let tempdir = tempdir()?;
    let cmd_path_result = which::which("docling");
    let cmd_path = match cmd_path_result {
        Ok(cmd_path) => cmd_path,
        Err(_) => return Err(Error::CantFindDoclings)
    };
    let mut cmd = Command::new(cmd_path);
    cmd.current_dir(tempdir.path());
    cmd.arg(pdf_abs_path);
    cmd.status()?;
    // get all child files
    let mut files = vec![];
    for entry in tempdir.path().read_dir()? {
        let entry = entry?;
        let path = entry.path();
        println!("Path: {:?}", path);
        if path.is_file() {
            files.push(path);
        }
    }
    if files.len() != 1 {
        return Err(Error::TooManyFilesFromPDF(files, from_pdf.display().to_string()));
    }
    let read = std::fs::read(&files[0])?;
    let cow_string = String::from_utf8_lossy(&read);

    Ok(cow_string.parse()?)
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