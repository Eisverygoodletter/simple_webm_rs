use crate::{WebmFrame, WebmFrameType};
fn yuv_clamp(x: i32) -> u8 {
    x.min(255).max(0) as u8
}
impl WebmFrame {
    pub fn to_argb(&mut self) {
        match self.frame_type {
            WebmFrameType::ARGB => {}
            WebmFrameType::RGBA => {
                self.rgba_to_argb();
            }
            WebmFrameType::I420 => {
                panic!("I420 to argb not implemented");
            }
        }
    }
    pub fn to_rgba(&mut self) {
        match self.frame_type {
            WebmFrameType::ARGB => {
                self.argb_to_rgba();
            }
            WebmFrameType::I420 => {
                unimplemented!("I420 to rgba not implemented");
            }
            WebmFrameType::RGBA => {}
        }
    }
    pub fn rgba_to_argb(&mut self) {
        if self.frame_type != WebmFrameType::RGBA {
            panic!("rgba_to_argb failed due to incorrect frame type");
        }
        let stride = self.buffer.len() / self.height;
        for y in 0..self.height {
            for x in 0..self.width {
                let o = y * stride + 4 * x;
                // rgba -> argb
                // swap two at a time so that
                // rgba -> rgab -> ragb -> argb
                self.buffer.swap(o + 2, o + 3);
                self.buffer.swap(o + 1, o + 2);
                self.buffer.swap(o, o + 1);
            }
        }
        self.frame_type = WebmFrameType::ARGB;
    }
    pub fn argb_to_rgba(&mut self) {
        if self.frame_type != WebmFrameType::ARGB {
            panic!("argb_to_rgba failed due to incorrect frame type");
        }
        let stride = self.buffer.len() / self.height;
        for y in 0..self.height {
            for x in 0..self.width {
                let o = y * stride + 4 * x;
                // argb -> rgba
                // swap two at a time so that
                // argb -> ragb -> rgab -> rgba
                self.buffer.swap(o, o + 1);
                self.buffer.swap(o + 1, o + 2);
                self.buffer.swap(o + 2, o + 3);
            }
        }
        self.frame_type = WebmFrameType::RGBA;
    }
    pub fn argb_to_i420(&mut self) {
        if self.frame_type != WebmFrameType::ARGB {
            panic!("argb_to_i420 failed due to incorrect frame type");
        }
        let mut target: Vec<u8> = Vec::new();
        let stride = self.buffer.len() / self.height;

        for y in 0..self.height {
            for x in 0..self.width {
                let o = y * stride + 4 * x;

                let b = self.buffer[o] as i32;
                let g = self.buffer[o + 1] as i32;
                let r = self.buffer[o + 2] as i32;

                let y = (66 * r + 129 * g + 25 * b + 128) / 256 + 16;
                target.push(yuv_clamp(y));
            }
        }

        for y in (0..self.height).step_by(2) {
            for x in (0..self.width).step_by(2) {
                let o = y * stride + 4 * x;

                let b = self.buffer[o] as i32;
                let g = self.buffer[o + 1] as i32;
                let r = self.buffer[o + 2] as i32;

                let u = (-38 * r - 74 * g + 112 * b + 128) / 256 + 128;
                target.push(yuv_clamp(u));
            }
        }

        for y in (0..self.height).step_by(2) {
            for x in (0..self.width).step_by(2) {
                let o = y * stride + 4 * x;

                let b = self.buffer[o] as i32;
                let g = self.buffer[o + 1] as i32;
                let r = self.buffer[o + 2] as i32;

                let v = (112 * r - 94 * g - 18 * b + 128) / 256 + 128;
                target.push(yuv_clamp(v));
            }
        }
        self.buffer = target;
        self.frame_type = WebmFrameType::I420;
    }
}