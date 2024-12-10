pub(crate) mod youtube;

use std::path::Path;
use crate::file::{FileType, InputType, WebsiteType};
use crate::file::error::ParseError;

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
    if let Err(_) = reqwest::blocking::get(input.to_string()) {
        return Err(ParseError::WebsiteNotReachable(input.to_string()))
    }

    if input.starts_with("https://youtube.com/") {
        return Ok(WebsiteType::Youtube(youtube::parse_youtube(input)?));
    }
    else {
        return Ok(WebsiteType::Article(input.to_string()));
    }
}


#[cfg(test)]
mod test_input_parse {
    use super::*;

    #[test]
    fn test_parse_youtube_video() {
        let input = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
        let result = parse_website(input);
    }
}