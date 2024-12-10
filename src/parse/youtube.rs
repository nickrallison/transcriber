use crate::error::ParseError;

use regex::Regex;
use lazy_static::lazy_static;
use rstest::rstest;



const YOUTUBE_VIDEO_URL: &str = "https://www.youtube.com/watch?v=";
const YOUTUBE_PLAYLIST_URL: &str = "https://www.youtube.com/playlist?list=";
const YOUTUBE_CHANNEL_URL: &str = "https://www.youtube.com/@";

lazy_static! {
    static ref YOUTUBE_VIDEO_REGEX: Regex = Regex::new(r"(?:https?://)?(?:www\.)?(?:youtube|youtu|youtube-nocookie)\.(?:com|be)/(?:watch\?v=|embed/|v/|.+\?v=)?(?P<id>[^&=%\?]*)").unwrap();
    static ref YOUTUBE_PLAYLIST_REGEX: Regex = Regex::new(r"(?:https?://)?(?:www\.)?(?:youtube|youtu|youtube-nocookie)\.(?:com|be)/(?:playlist\?list=|embed/|v/|.+\?list=)?(?P<id>[^&=%\?]*)").unwrap();
    static ref YOUTUBE_CHANNEL_REGEX: Regex = Regex::new(r"(?:https?://)?(?:www\.)?(?:youtube|youtu|youtube-nocookie)\.(?:com|be)/(?:channel/|user/|@)?(?P<id>[^&=%\?]*)").unwrap();
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum YoutubeType {
    Video(String),
    Playlist(String),
    Channel(String),
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

    #[rstest]
    #[case("https://www.youtube.com/watch?v=dQw4w9WgXcQ", "dQw4w9WgXcQ")]
    #[case("https://www.youtube.com/watch?v=n43zQvMdPmE", "n43zQvMdPmE")]
    #[case("https://www.youtube.com/watch?v=8xboFsRijUk", "8xboFsRijUk")]
    #[case("https://www.youtube.com/watch?v=crMyNv2fdkc&list=PLUl4u3cNGP61cYB5ymvFiEbIb-wWHfaqO", "crMyNv2fdkc")]
    #[case("https://www.youtube.com/watch?v=L3LMbpZIKhQ&list=PLB7540DEDD482705B&index=1&pp=iAQB", "L3LMbpZIKhQ")]
    #[case("https://www.youtube.com/watch?v=CMzCH_P_SFI", "CMzCH_P_SFI")]
    fn youtube_video(#[case] link: &str, #[case] expected_id: &str) {
        let result = parse_youtube(link);
        match result {
            Ok(_) => (),
            Err(e) => panic!("{}", e)
        }
        let youtube_type = result.unwrap();
        match youtube_type {
            YoutubeType::Video(video_id) => assert_eq!(expected_id, video_id),
            YoutubeType::Playlist(_) => panic!("{}", &format!("Link parsed as playlist: {}", &link)),
            YoutubeType::Channel(_) => panic!("{}", &format!("Link parsed as channel: {}", &link)),
        }
    }

    #[rstest]
    #[case("https://www.youtube.com/playlist?list=PLUl4u3cNGP61cYB5ymvFiEbIb-wWHfaqO", "PLUl4u3cNGP61cYB5ymvFiEbIb-wWHfaqO")]
    #[case("https://www.youtube.com/playlist?list=PLVTclEEyY1SKFumpT86h-y6jikkEUKIAH", "PLVTclEEyY1SKFumpT86h-y6jikkEUKIAH")]
    #[case("https://www.youtube.com/playlist?list=PLPKF63qhXeX3-E-PL08f3CKN4CPfT7nLv", "PLPKF63qhXeX3-E-PL08f3CKN4CPfT7nLv")]
    fn youtube_playlist(#[case] link: &str, #[case] expected_id: &str) {
        let result = parse_youtube(link);
        match result {
            Ok(_) => (),
            Err(e) => panic!("{}", e)
        }
        let youtube_type = result.unwrap();
        match youtube_type {
            YoutubeType::Playlist(playlist_id) => assert_eq!(expected_id, playlist_id),
            YoutubeType::Video(_) => panic!("{}", &format!("Link parsed as video: {}", &link)),
            YoutubeType::Channel(_) => panic!("{}", &format!("Link parsed as channel: {}", &link)),
        }
    }

    #[rstest]
    #[case("https://www.youtube.com/@JackChappleShow", "JackChappleShow")]
    #[case("https://www.youtube.com/@mitocw", "mitocw")]
    #[case("https://www.youtube.com/@aiexplained-official", "aiexplained-official")]
    #[case("https://www.youtube.com/@AbroadinJapan", "AbroadinJapan")]
    fn youtube_channel(#[case] link: &str, #[case] expected_id: &str) {
        let result = parse_youtube(link);
        match result {
            Ok(_) => (),
            Err(e) => panic!("{}", e)
        }
        let youtube_type = result.unwrap();
        match youtube_type {
            YoutubeType::Channel(channel_id) => assert_eq!(expected_id, channel_id),
            YoutubeType::Video(_) => panic!("{}", &format!("Link parsed as video: {}", &link)),
            YoutubeType::Playlist(_) => panic!("{}", &format!("Link parsed as playlist: {}", &link)),
        }
    }

    #[test]
    fn youtube_invalid() {
        let result = parse_youtube("https://www.youtube.com");
        match result {
            Ok(_) => panic!("Youtube parse sShould have failed"),
            Err(e) => {
                match e {
                    ParseError::YoutubeLinkInvalid(_) => (),
                    _ => panic!("{}", &format!("Unexpected error: {}", e))
                }
            }
        }
    }
}
