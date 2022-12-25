#[cfg(test)]
mod tests {
    use image::ImageBuffer;
    use image::{Rgba, RgbaImage};
    use simple_webm::{WebmFrame, WebmFrameType};

    fn get_test_img() -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut test_img = RgbaImage::new(32, 32);
        for x in 15..=17 {
            for y in 8..24 {
                test_img.put_pixel(x, y, Rgba([255, 0, 0, 255]));
                test_img.put_pixel(y, x, Rgba([255, 0, 0, 255]));
            }
        }
        test_img.put_pixel(0, 0, Rgba([0, 255, 0, 255]));
        test_img
    }

    #[test]
    fn rgba_argb_conversion() {
        let _ = std::fs::remove_dir_all("tests/output/rgba_argb_conversion/");
        let _ = std::fs::create_dir_all("tests/output/rgba_argb_conversion/");
        let test_img = get_test_img();
        let mut frame = WebmFrame::from_image_buffer_rgba(test_img);
        assert_eq!(frame.width, 32);
        assert_eq!(frame.height, 32);
        assert_eq!(frame.frame_type, WebmFrameType::RGBA);
        let mut original = frame.clone();
        frame.to_argb();
        frame.to_rgba();
        let _ = original.save_as_file("tests/output/rgba_argb_conversion/original.png");
        let _ = frame.save_as_file("tests/output/rgba_argb_conversion/modified.png");
        assert_eq!(original, frame);
    }
    #[test]
    fn i420_argb_conversion() {
        let _ = std::fs::remove_dir_all("tests/output/i420_argb_conversion/");
        let _ = std::fs::create_dir_all("tests/output/i420_argb_conversion/");
        let test_img = get_test_img();
        let mut frame = WebmFrame::from_image_buffer_rgba(test_img);
        let mut original = frame.clone();
        frame.to_argb();
        frame.to_i420();
        frame.to_argb();
        frame.to_rgba();
        let _ = original.save_as_file("tests/output/i420_argb_conversion/original.png");
        let _ = frame.save_as_file("tests/output/i420_argb_conversion/frame.png");
        // unfortunately, converting to YUV and back to rgba/argb colour formats loses information
        // YUV does not store alpha information
        // we simply have to check the 2 pngs saved to see if there were any errors
    }
}
