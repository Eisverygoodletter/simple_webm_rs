#[cfg(test)]
mod tests {
    use image::ImageBuffer;
    use image::{Rgba, RgbaImage};
    use simple_webm::{WebmFrame, WebmFrameType};

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
        std::fs::create_dir("tests/output/");
        let test_img = get_test_img();
        let mut frame = WebmFrame::from_image_buffer_rgba(test_img);
        assert_eq!(frame.width, 32);
        assert_eq!(frame.height, 32);
        assert_eq!(frame.frame_type, WebmFrameType::RGBA);
        let mut original = frame.clone();
        original
            .save_as_file("tests/output/rgba_argb_conversion_original.png")
            .unwrap();
        frame.to_argb();
        assert_eq!(frame.frame_type, WebmFrameType::ARGB);
        //frame.to_rgba();
        frame
            .save_as_file("tests/output/rgba_argb_conversion_modified.png")
            .unwrap();
        assert_eq!(original, frame);
    }
}
