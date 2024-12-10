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

use crate::{FileCategory, FileType, InputType, StringFile, WebsiteType};
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
        WebsiteType::Article(article) => vec![transform_article(article)?],
        WebsiteType::Youtube(youtube) => transform_youtube(youtube)?
    };
    Ok(files)
}

fn transform_article(website_url: String) -> Result<crate::FileType, error::TransformError> {

    let url = Url::parse(&website_url)?;
    // wget the site with reqwest
    let mut response = match url.scheme() {
        "http" => {
            let client = reqwest::blocking::Client::new();
            client.get(website_url.as_str())
                .send()?
        },
        "https" => {
            let client = reqwest::blocking::Client::builder()
                .danger_accept_invalid_certs(true)
                .build()?;
            client.get(website_url.as_str())
                .send()?
        },
        _ => {
            return Err(crate::transform::error::TransformError::UrlScheme(format!("Invalid url scheme: {}", url.scheme())))
        }
    };

    let mut html = String::new();
    let filepath: PathBuf = url.path().parse()?;

    let filename_opt = filepath.file_name();
    if filename_opt.is_none() {
        return Err(crate::transform::error::TransformError::Filename(format!("Filename not found for {}", website_url)))
    }
    let filename: OsString = filename_opt.expect("Filename not found for link").to_os_string();
    
    response.read_to_string(&mut html)?;
    let file: FileType = FileType::StringFile(
        StringFile {
            file_name: filename,
            contents: html,
            file_type: FileCategory::Html,
        }
    );
    Ok(file)
}

#[cfg(test)]
mod transform_tests {
    


}