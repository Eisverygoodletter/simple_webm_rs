use simple_webm::{add, WebmFrame};
use simple_webm::image_loading;
use simple_webm::convert_frames;

use image::{RgbaImage, Rgba};
#[cfg(test)]
mod tests {
    use image::ImageBuffer;
    use simple_webm::WebmFrameType;

    use super::*;

    fn get_test_img() -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut test_img = RgbaImage::new(32, 32);
        for x in 15..=17 {
            for y in 8..24 {
                test_img.put_pixel(x, y, Rgba([255, 0, 0, 255]));
                test_img.put_pixel(y, x, Rgba([255, 0, 0, 255]));
            }
        }
        test_img
    }

    #[test]
    fn rgba_argb_conversion() {
        let mut test_img = get_test_img();
        let mut frame = WebmFrame::from_image_buffer_rgba(test_img);
        assert_eq!(frame.width, 32);
        assert_eq!(frame.height, 32);
        assert_eq!(frame.frame_type, WebmFrameType::RGBA);
        let original = frame.clone();
        frame.to_argb();
        assert_eq!(frame.frame_type, WebmFrameType::ARGB);
        frame.argb_to_rgba();
        assert_eq!(frame, original);
        
    }
}
