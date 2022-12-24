extern crate webm;
use webm::mux::{self, VideoCodecId};
use std::io::Cursor;
pub mod convert_frames;
#[cfg(feature = "image_loading")]
pub mod image_loading;

#[derive(PartialEq, Debug, Clone)]
pub enum WebmFrameType {
    ARGB,
    I420,
    RGBA,
}
#[derive(Clone, PartialEq, Debug)]
pub struct WebmFrame {
    buffer: Vec<u8>,
    pub frame_type: WebmFrameType,
    pub width: usize,
    pub height: usize
}
pub struct WebmVideo {
    segment: mux::Segment<mux::Writer<Cursor<Vec<u8>>>>

}
impl WebmVideo {
    fn length(&mut self) {
        let a = self.segment.add_video_track(100, 100, Some(0), VideoCodecId::VP9);
    }
}



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

