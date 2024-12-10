use crate::file::error::ParseError;

use regex::Regex;
use lazy_static::lazy_static;



const YOUTUBE_VIDEO_URL: &str = "https://www.youtube.com/watch?v=";
const YOUTUBE_PLAYLIST_URL: &str = "https://www.youtube.com/playlist?list=";
const YOUTUBE_CHANNEL_URL: &str = "https://www.youtube.com/@";

lazy_static! {
    static ref YOUTUBE_VIDEO_REGEX: Regex = Regex::new(r"(?:https?://)?(?:www\.)?(?:youtube|youtu|youtube-nocookie)\.(?:com|be)/(?:watch\?v=|embed/|v/|.+\?v=)?(?P<id>[^&=%\?].*)").unwrap();
    static ref YOUTUBE_PLAYLIST_REGEX: Regex = Regex::new(r"(?:https?://)?(?:www\.)?(?:youtube|youtu|youtube-nocookie)\.(?:com|be)/(?:playlist\?list=|embed/|v/|.+\?list=)?(?P<id>[^&=%\?].*)").unwrap();
    static ref YOUTUBE_CHANNEL_REGEX: Regex = Regex::new(r"(?:https?://)?(?:www\.)?(?:youtube|youtu|youtube-nocookie)\.(?:com|be)/(?:channel/|user/)?(?P<id>[^&=%\?].*)").unwrap();
}

pub enum YoutubeType {
    Video(String),
    Playlist(String),
    Channel(String)
}

pub fn parse_youtube(input: &str) -> Result<YoutubeType, ParseError> {
    if input.starts_with(YOUTUBE_VIDEO_URL) {
	    return Ok(YoutubeType::Video(extract_video_id(input)?))
    }
    if input.starts_with(YOUTUBE_PLAYLIST_URL) {
        return Ok(YoutubeType::Playlist(extract_playlist_id(input)?))
    }
    if input.starts_with(YOUTUBE_CHANNEL_URL) {
        return Ok(YoutubeType::Channel(extract_channel_id(input)?))
    }
    return Err(ParseError::YoutubeLinkInvalid(input.to_string()))

}

fn extract_video_id(url: &str) -> Result<String, ParseError> {
    match YOUTUBE_VIDEO_REGEX.captures(&url) {
        Some(caps) => Ok(caps["id"].to_string()),
        None => Err(ParseError::YoutubeRegexFail(url.to_string()))
    }
}

fn extract_playlist_id(url: &str) -> Result<String, ParseError> {
    match YOUTUBE_PLAYLIST_REGEX.captures(&url) {
        Some(caps) => Ok(caps["id"].to_string()),
        None  => Err(ParseError::YoutubeRegexFail(url.to_string()))
     }
}
fn extract_channel_id(url: &str) -> Result<String, ParseError> {
    match YOUTUBE_CHANNEL_REGEX.captures(&url)  {
        Some(caps) => Ok(caps["id"].to_string()),
        None   => Err(ParseError::YoutubeRegexFail(url.to_string()))
      }
}

#[cfg(test)]
mod parse_youtube_tests {
    use super::*;
    #[test]
    fn youtube_video() {
        let link = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
        let result = parse_youtube(link);
        match result {
            Ok(_) => (),
            Err(e) => panic!("{}", e)
        }
        let youtube_type = result.unwrap();
        match youtube_type {
            YoutubeType::Video(video_id) => assert_eq!("dQw4w9WgXcQ", video_id),
            YoutubeType::Playlist(_) => panic!("{}", &format!("Link parsed as playlist: {}", &link)),
            YoutubeType::Channel(_) => panic!("{}", &format!("Link parsed as channel: {}", &link)),
        }
    }
    #[test]
    fn youtube_playlist() {
        let link = "https://www.youtube.com/playlist?list=PLB7540DEDD482705B";
        let result = parse_youtube(link);
        match result {
            Ok(_) => (),
            Err(e) => panic!("{}", e)
        }
        let youtube_type = result.unwrap();
        match youtube_type {
            YoutubeType::Playlist(playlist_id) => assert_eq!("PLB7540DEDD482705B", playlist_id),
            YoutubeType::Video(_) => panic!("{}", &format!("Link parsed as video: {}", &link)),
            YoutubeType::Channel(_) => panic!("{}", &format!("Link parsed as channel: {}", &link)),
        }
    }
}
