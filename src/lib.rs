extern crate webm;
// use std::io::Cursor;
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
    pub buffer: Vec<u8>,
    pub frame_type: WebmFrameType,
    pub width: usize,
    pub height: usize,
}
// todo
// pub struct WebmVideo {
//     segment: mux::Segment<mux::Writer<Cursor<Vec<u8>>>>,
// }
