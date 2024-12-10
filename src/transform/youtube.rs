//! Youtube Management for Transformation Lib
//! TODO: Add Documentation to crate::transform::youtube
//! TODO: Add Tests to crate::transform::youtube
//! TODO: Add Debug Asserts to crate::transform::youtube

use crate::FileType;
use crate::parse::youtube::YoutubeType;

/// Transforms youtube enum into associated file
pub fn transform_youtube(youtube_type: YoutubeType) -> Result<crate::FileType, crate::transform::error::TransformError> {
    // require yt-dlp to be installed
    which::which("yt-dlp")?;

    let file: FileType = match youtube_type {
        YoutubeType::Video(vid_id) => transform_youtube_video(vid_id)?,
        YoutubeType::Playlist(playlist_id) => transform_youtube_playlist(playlist_id)?,
        YoutubeType::Channel(channel_id) => transform_youtube_channel(channel_id)?,
    };

    Ok(file)
}

/// Grabs a youtube
fn transform_youtube_video(vid_id: String) -> Result<crate::FileType, crate::transform::error::TransformError> {
    todo!()
}

fn transform_youtube_playlist(playlist_id: String) -> Result<crate::FileType, crate::transform::error::TransformError> {
    todo!()
}

fn transform_youtube_channel(channel_id: String) -> Result<crate::FileType, crate::transform::error::TransformError> {
    todo!()
}
