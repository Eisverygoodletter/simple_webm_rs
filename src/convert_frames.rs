use crate::{buffer_conversions::ColorConvertableBuffer, WebmFrame, WebmFrameType};

impl WebmFrame {
    pub fn to_argb(&mut self) {
        match self.frame_type {
            WebmFrameType::ARGB => {}
            WebmFrameType::RGBA => {
                self.buffer = self.buffer.rgba_to_argb(self.width, self.height);
            }
            WebmFrameType::I420 => {
                self.buffer = self.buffer.i420_to_argb(self.width, self.height);
            }
        }
        self.frame_type = WebmFrameType::ARGB;
    }
    pub fn to_rgba(&mut self) {
        match self.frame_type {
            WebmFrameType::ARGB => {
                self.buffer = self.buffer.argb_to_rgba(self.width, self.height);
            }
            WebmFrameType::I420 => {
                unimplemented!("I420 to rgba not implemented");
            }
            WebmFrameType::RGBA => {
                println!("no?");
            }
        }
        self.frame_type = WebmFrameType::RGBA;
    }
    pub fn to_i420(&mut self) {
        match self.frame_type {
            WebmFrameType::ARGB => self.buffer = self.buffer.argb_to_i420(self.width, self.height),
            _ => {
                unimplemented!("to_i420 not completely implemented")
            }
        }
        self.frame_type = WebmFrameType::I420;
    }
}
