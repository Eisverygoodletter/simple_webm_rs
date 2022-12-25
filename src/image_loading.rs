use crate::{WebmFrame, WebmFrameType};
use image::{ImageBuffer, ImageError, Rgba};
impl WebmFrame {
    pub fn from_image_buffer_rgba(image: image::RgbaImage) -> WebmFrame {
        let frame = WebmFrame {
            buffer: image.as_raw().to_owned(),
            frame_type: WebmFrameType::RGBA,
            width: image.width() as usize,
            height: image.height() as usize,
        };
        frame
    }
    pub fn save_as_file(&mut self, filename: &str) -> Result<(), ImageError> {
        self.to_rgba();
        // let buf = &self.buffer;

        let image_opt: Option<ImageBuffer<Rgba<u8>, Vec<u8>>> =
            ImageBuffer::from_raw(self.width as u32, self.height as u32, self.buffer.clone());
        let image = image_opt.unwrap();
        image.save(filename)
    }
}
