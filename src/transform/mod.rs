//! Transfomation Lib
//! TODO: Add Documentation to crate::transform
//! TODO: Add Tests to crate::transform
//! TODO: Add Debug Asserts to crate::transform

mod youtube;

use std::fs::File;
use std::io::{Read, Write};

use crate::{FileType, InputType, WebsiteType};
use crate::transform::youtube::transform_youtube;

pub fn transform_input(input: InputType) -> Result<Vec<FileType>, crate::error::Error> {
    let files: Vec<FileType> = match input {
        InputType::File(file) => vec![file],
        InputType::Website(website) => transform_website(website)?
    };
    Ok(files)
}

fn transform_website(website: WebsiteType) -> Result<Vec<crate::FileType>, crate::error::Error> {
    let files: Vec<FileType> = match website {
        WebsiteType::Article(article) => vec![transform_article(&article)?],
        WebsiteType::Youtube(youtube) => transform_youtube(youtube)?
    };
    Ok(files)
}

fn transform_article(website_url: &str) -> Result<crate::FileType, crate::error::Error> {
    let mut response = reqwest::blocking::get(website_url)?;

    // Extract the file name from the URL or use a default name if extraction fails
    let filename_opt = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) });
    if filename_opt.is_none() {
        return Err(crate::error::Error::MissingFileName(website_url.to_string()));
    }
    let filename = filename_opt.unwrap();
    let tempdir = std::env::temp_dir();
    let filepath = tempdir.join(filename);
    let mut file = File::create(&filepath)?;
    std::io::copy(&mut response, &mut file).expect("failed to copy content");
    Ok(FileType::new_path(filepath)?)
}

#[cfg(test)]
mod transform_tests {
    use rstest::rstest;

    #[rstest]
    #[case("https://users.rust-lang.org/t/how-to-download-files-from-the-internet/54878/3", crate::FileCategory::Html)]
    #[case("https://personal.utdallas.edu/~gxm112130/papers/iscas15.pdf", crate::FileCategory::Pdf)]
    fn test_transform_article(#[case] url: &str, #[case] file_category: crate::FileCategory) {
        let file = crate::transform::transform_article(url);
        assert_eq!(file.unwrap().category(), file_category);

    }

}