use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    // #[error("Input was not a valid link and not a file that exists")]
    // InvalidInput(String),

    #[error("Web page did not return a valid response: {0}")]
    WebsiteNotReachable(String),

    #[error("Could not parse the youtube link into a video, playlist or channel: {0}")]
    YoutubeLinkInvalid(String),

    #[error("Could not parse youtube link with regex")]
    YoutubeRegexFail(String),
}