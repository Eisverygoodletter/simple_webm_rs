extern crate webm;
// use std::io::Cursor;
mod buffer_conversions;
pub mod convert_frames;
#[cfg(feature = "image_loading")]
pub mod image_loading;

/// Specifies the colour encoding of a WebmFrame\
/// The [`Image`] crate from Rust uses RGBA, but [`libyuv`] which is used for conversions favours their "ARGB".\
/// The [`Image`] crate's RGBA is actually the same as [`libyuv`]'s "ARGB" due to how they are laid out in memory.\
/// Whenever this crate refers to "RGBA", it is referring to [`Image`](https://docs.rs/image/0.24.5/image/) crate's RGBA and `libyuv`'s "ARGB".
/// 
/// [`Image`]: https://docs.rs/image/0.24.5/image/
/// [`libyuv`]: https://chromium.googlesource.com/libyuv/libyuv/+/refs/heads/main/
#[derive(PartialEq, Debug, Clone)]
pub enum WebmFrameType {
    /// ARGB - Alpha, Red, Green, Blue
    ARGB,
    /// [YUV](https://en.wikipedia.org/wiki/YUV) - Luminance, Blue projection, Red projection
    I420,
    RGBA,
}

/// Represents a frame (Video, not audio) in a Webm video
#[derive(Clone, PartialEq, Debug)]
pub struct WebmFrame {
    pub buffer: Vec<u8>,
    pub frame_type: WebmFrameType,
    pub width: usize,
    pub height: usize,
}

use webm::mux;
use std::io::Cursor;
/// Represents a webm video
pub struct WebmVideo {
    segment: mux::Segment<mux::Writer<Cursor<Vec<u8>>>>,
    video_tracks: mux::VideoTrack,
    audio_tracks: mux::AudioTrack
}
