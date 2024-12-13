//! Youtube Management for Parsing Lib
//! TODO: Add Documentation to crate::parse::youtube
//! TODO: Add Tests to crate::parse::youtube
//! TODO: Add Debug Asserts to crate::parse::youtube

use crate::error::Error;

use regex::Regex;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

const YOUTUBE_VIDEO_URL: &str = "https://www.youtube.com/watch?v=";
const YOUTUBE_PLAYLIST_URL: &str = "https://www.youtube.com/playlist?list=";
const YOUTUBE_CHANNEL_AT_URL: &str = "https://www.youtube.com/@";
const YOUTUBE_CHANNEL_URL: &str = "https://www.youtube.com/user";

const YOUTUBE_PREFIXES: [&str; 4] = [YOUTUBE_VIDEO_URL, YOUTUBE_PLAYLIST_URL, YOUTUBE_CHANNEL_AT_URL, YOUTUBE_CHANNEL_URL];

lazy_static! {
    static ref YOUTUBE_VIDEO_REGEX: Regex = Regex::new(r"(?:https?://)?(?:www\.)?(?:youtube|youtu|youtube-nocookie)\.(?:com|be)/(?:watch\?v=|embed/|v/|.+\?v=)?(?P<id>[^&=%\?]*)").unwrap();
    static ref YOUTUBE_PLAYLIST_REGEX: Regex = Regex::new(r"(?:https?://)?(?:www\.)?(?:youtube|youtu|youtube-nocookie)\.(?:com|be)/(?:playlist\?list=|embed/|v/|.+\?list=)?(?P<id>[^&=%\?]*)").unwrap();
    static ref YOUTUBE_CHANNEL_REGEX: Regex = Regex::new(r"(?:https?://)?(?:www\.)?(?:youtube|youtu|youtube-nocookie)\.(?:com|be)/(?:channel/|user/)?(?P<id>[^&=%\?]*)").unwrap();
    static ref YOUTUBE_CHANNEL_AT_REGEX: Regex = Regex::new(r"(?:https?://)?(?:www\.)?(?:youtube|youtu|youtube-nocookie)\.(?:com|be)/(?:@)?(?P<id>[^&=%\?]*)").unwrap();
}

#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub enum YoutubeType {
    Video(String),
    Playlist(String),
    Channel(String),
    ChannelAt(String),
}

pub fn parse_youtube(input: &str) -> Result<YoutubeType, Error> {
    if input.starts_with(YOUTUBE_VIDEO_URL) {
	    return Ok(YoutubeType::Video(extract_video_id(input)?))
    }
    if input.starts_with(YOUTUBE_PLAYLIST_URL) {
        return Ok(YoutubeType::Playlist(extract_playlist_id(input)?))
    }
    if input.starts_with(YOUTUBE_CHANNEL_URL) {
        return Ok(YoutubeType::Channel(extract_channel_id(input)?))
    }
    if input.starts_with(YOUTUBE_CHANNEL_AT_URL) {
        return Ok(YoutubeType::ChannelAt(extract_channel_id_at(input)?))
    }
    Err(Error::YoutubeLinkParse(input.to_string(), &YOUTUBE_PREFIXES))

}

fn extract_video_id(url: &str) -> Result<String, Error> {
    match YOUTUBE_VIDEO_REGEX.captures(url) {
        Some(caps) => Ok(caps["id"].to_string()),
        None => Err(Error::YoutubeRegex(YOUTUBE_VIDEO_REGEX.clone(), "video".to_string(), url.to_string()))
    }
}

fn extract_playlist_id(url: &str) -> Result<String, Error> {
    match YOUTUBE_PLAYLIST_REGEX.captures(url) {
        Some(caps) => Ok(caps["id"].to_string()),
        None  => Err(Error::YoutubeRegex(YOUTUBE_PLAYLIST_REGEX.clone(), "playlist".to_string(), url.to_string()))
     }
}
fn extract_channel_id(url: &str) -> Result<String, Error> {
    match YOUTUBE_CHANNEL_REGEX.captures(url)  {
        Some(caps) => Ok(caps["id"].to_string()),
        None   => Err(Error::YoutubeRegex(YOUTUBE_CHANNEL_REGEX.clone(), "channel".to_string(), url.to_string()))
      }
}

fn extract_channel_id_at(url: &str) -> Result<String, Error> {
    match YOUTUBE_CHANNEL_AT_REGEX.captures(url)  {
        Some(caps) => Ok(caps["id"].to_string()),
        None   => Err(Error::YoutubeRegex(YOUTUBE_CHANNEL_AT_REGEX.clone(), "channel".to_string(), url.to_string()))
    }
}

#[cfg(test)]
mod parse_youtube_tests {
    use rstest::rstest;
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
            YoutubeType::ChannelAt(_) => panic!("{}", &format!("Link parsed as channel @: {}", &link)),
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
            YoutubeType::ChannelAt(_) => panic!("{}", &format!("Link parsed as channel @: {}", &link)),
        }
    }

    #[rstest]
    #[case("https://www.youtube.com/@JackChappleShow", "JackChappleShow")]
    #[case("https://www.youtube.com/@mitocw", "mitocw")]
    #[case("https://www.youtube.com/@aiexplained-official", "aiexplained-official")]
    #[case("https://www.youtube.com/@AbroadinJapan", "AbroadinJapan")]
    #[case("https://www.youtube.com/user/TheLinuxFoundation", "TheLinuxFoundation")]
    fn youtube_channel(#[case] link: &str, #[case] expected_id: &str) {
        let result = parse_youtube(link);
        match result {
            Ok(_) => (),
            Err(e) => panic!("{}", e)
        }
        let youtube_type = result.unwrap();
        match youtube_type {
            YoutubeType::ChannelAt(channel_id) => assert_eq!(expected_id, channel_id),
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
                    Error::YoutubeLinkParse(_, _) => (),
                    _ => panic!("{}", &format!("Unexpected error: {}", e))
                }
            }
        }
    }
}
