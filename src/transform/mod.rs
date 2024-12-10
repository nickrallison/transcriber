pub(crate) mod error;
mod youtube;

use std::io::Read;
use url::Url;

use crate::{FileCategory, FileType, InputType, StringFile, WebsiteType};
use crate::transform::youtube::transform_youtube;

pub fn transform_input(input: InputType) -> Result<crate::FileType, crate::transform::error::TransformError> {
    let file = match input {
        InputType::File(file) => file,
        InputType::Website(website) => transform_website(website)?
    };
    Ok(file)
}

fn transform_website(website: WebsiteType) -> Result<crate::FileType, crate::transform::error::TransformError> {
    let file = match website {
        WebsiteType::Article(article) => transform_article(article)?,
        WebsiteType::Youtube(youtube) => transform_youtube(youtube)?,
    };
    Ok(file)
}

fn transform_article(website_url: String) -> Result<crate::FileType, crate::transform::error::TransformError> {

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
    response.read_to_string(&mut html)?;
    let file: FileType = FileType::StringFile(
        StringFile {
            contents: html,
            file_type: FileCategory::Html,
        }
    );
    Ok(file)
}

#[cfg(test)]
mod transform_tests {
    use super::*;


}