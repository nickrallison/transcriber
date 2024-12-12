//! Transfomation Lib
//! TODO: Add Documentation to crate::transform
//! TODO: Add Tests to crate::transform
//! TODO: Add Debug Asserts to crate::transform

pub(crate) mod error;
mod youtube;

use std::ffi::OsString;
use std::io::Read;
use std::path::PathBuf;
use url::Url;

use crate::{BytesFile, FileCategory, FileType, InputType, StringFile, WebsiteType};
use crate::transform::youtube::transform_youtube;

pub fn transform_input(input: InputType) -> Result<Vec<FileType>, error::TransformError> {
    let files: Vec<FileType> = match input {
        InputType::File(file) => vec![file],
        InputType::Website(website) => transform_website(website)?
    };
    Ok(files)
}

fn transform_website(website: WebsiteType) -> Result<Vec<crate::FileType>, error::TransformError> {
    let files: Vec<FileType> = match website {
        WebsiteType::Article(article) => vec![transform_article(&article)?],
        WebsiteType::Youtube(youtube) => transform_youtube(youtube)?
    };
    Ok(files)
}

fn transform_article(website_url: &str) -> Result<crate::FileType, error::TransformError> {

    let url = Url::parse(&website_url)?;
    // wget the site with reqwest
    let mut response = match url.scheme() {
        "http" => {
            let client = reqwest::blocking::Client::new();
            client.get(website_url)
                .send()?
        },
        "https" => {
            let client = reqwest::blocking::Client::builder()
                .danger_accept_invalid_certs(true)
                .build()?;
            client.get(website_url)
                .send()?
        },
        _ => {
            return Err(crate::transform::error::TransformError::UrlScheme(format!("Invalid url scheme: {}", url.scheme())))
        }
    };

    let mut html = String::new();
    let filepath: PathBuf = url.path().parse()?;
    // let filename = crate::get_filename(&filepath);

    let file_category = crate::util::get_file_type(&filepath);
    if file_category.is_err() {
        // normal html page
        response.read_to_string(&mut html)?;
        let filename: String = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .expect("should have had filename")
            .to_string();
        println!("{}", filename);
        let filename: OsString = OsString::from(filename);
        let file: FileType = FileType::StringFile(
            StringFile {
                file_name: filename,
                contents: html,
                file_type: FileCategory::Html,
            }
        );
        return Ok(file);
    }
    
    // Got and downloaded a valid file
    let filename_opt = &filepath.file_name();
    let filename: OsString = filename_opt.expect("Filename not found for link").to_os_string();
    let file_category = crate::util::get_file_type(&filepath);
    if file_category.is_err() {
        return Err(crate::transform::error::TransformError::FileCategory(format!("Invalid file category: {}", file_category.unwrap_err())));
    }
    let bytes: Vec<u8> = response.bytes()?.to_vec();
    let file: FileType = FileType::BytesFile(
        BytesFile {
            file_name: filename,
            bytes,
            file_type: file_category.unwrap(),
        }
    );
    Ok(file)

}

#[cfg(test)]
mod transform_tests {
    use rstest::rstest;

    #[rstest]
    #[case("https://users.rust-lang.org/t/how-to-download-files-from-the-internet/54878/3", crate::FileCategory::Html)]
    #[case("https://personal.utdallas.edu/~gxm112130/papers/iscas15.pdf", crate::FileCategory::Pdf)]
    fn test_tranform_article(#[case] url: &str, #[case] file_category: crate::FileCategory) {
        let file = crate::transform::transform_article(url);
        assert_eq!(file.unwrap().category(), file_category);

    }

}