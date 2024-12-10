pub(crate) mod youtube;

use crate::file::{FileType, InputType, WebsiteType};
use crate::file::error::ParseError;

use lazy_static::lazy_static;
use regex::Regex;
use std::path::{Path, PathBuf};

lazy_static! {
    static ref YOUTUBE_REGEX: Regex = Regex::new(r"(?:https?://)?(?:www\.)?(?:youtube|youtu|youtube-nocookie)\.(?:com|be)").unwrap();
}

pub fn parse_input(input: &str) -> Result<InputType, ParseError> {
    if input.contains("http") {
        return Ok(InputType::Website(parse_website(input)?));
    }
    return Ok(InputType::File(parse_file(input)?));

}

fn parse_file(input: &str) -> Result<FileType, ParseError> {
    todo!()
}

fn parse_website(input: &str) -> Result<WebsiteType, ParseError> {
    // if not able to ping website, return error
    // if let Err(_) = reqwest::blocking::get(input.to_string()) {
    //     return Err(ParseError::WebsiteNotReachable(input.to_string()))
    // }

    if YOUTUBE_REGEX.is_match(&input) {
        Ok(WebsiteType::Youtube(youtube::parse_youtube(input)?))
    }
    else {
        Ok(WebsiteType::Article(input.to_string()))
    }
}


#[cfg(test)]
mod test_input_parse {
    use super::*;

    #[test]
    fn test_parse_youtube_video() {
        let input = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
        let result = parse_website(input);
        match result {
            Ok(youtube) => {
                assert_eq!(youtube, WebsiteType::Youtube(youtube::YoutubeType::Video("dQw4w9WgXcQ".to_string())));
            }
            Err(_) => {
                panic!("parse youtube video failed")
            }
        }
    }


}