//! Conversions

pub(crate) trait ColorConvertableBuffer {
    fn argb_to_rgba(&self, width: usize, height: usize) -> Self;
    fn rgba_to_argb(&self, width: usize, height: usize) -> Self;
    fn i420_to_argb(&self, width: usize, height: usize) -> Self;
    fn argb_to_i420(&self, width: usize, height: usize) -> Self;
}

#[cfg(feature = "libyuv_conversion")]
mod buffer_conversion {
    use crate::buffer_conversions::ColorConvertableBuffer;
    use libyuv;
    use std::ffi::c_int;
    impl ColorConvertableBuffer for Vec<u8> {
        fn argb_to_rgba(&self, width: usize, height: usize) -> Vec<u8> {
            let mut target = vec![0; self.len()];
            unsafe {
                // rgba to argb
                let _c_res = libyuv::rgba_to_argb(
                    self.as_ptr(),
                    width as c_int * 4,
                    target.as_mut_ptr(),
                    width as c_int * 4,
                    width as c_int,
                    height as c_int,
                );
            }
            target
        }
        fn rgba_to_argb(&self, width: usize, height: usize) -> Vec<u8> {
            let mut target = vec![0; self.len()];
            unsafe {
                let _c_res = libyuv::argb_to_rgba(
                    self.as_ptr(),
                    width as c_int * 4,
                    target.as_mut_ptr(),
                    width as c_int * 4,
                    width as c_int,
                    height as c_int,
                );
            }
            target
        }
        fn i420_to_argb(&self, width: usize, height: usize) -> Self {
            let mut target = vec![0; width * height * 4];
            unsafe {
                let _c_res = libyuv::i420_to_rgba(
                    self.as_ptr(),
                    width as c_int,
                    self.as_ptr().offset((width * height) as isize),
                    (width / 2) as c_int,
                    self.as_ptr().offset((width * height * 5 / 4) as isize),
                    (width / 2) as c_int,
                    target.as_mut_ptr(),
                    (width * 4) as c_int,
                    width as c_int,
                    height as c_int,
                );
            }
            target
        }
        fn argb_to_i420(&self, width: usize, height: usize) -> Self {
            let mut target = vec![0; width * height + width * height / 4 * 2];
            unsafe {
                let _c_res = libyuv::rgba_to_i420(
                    self.as_ptr(),
                    (width * 4) as c_int,
                    target.as_mut_ptr(),
                    width as c_int,
                    target.as_mut_ptr().offset((width * height) as isize),
                    (width / 2) as c_int,
                    target
                        .as_mut_ptr()
                        .offset((width * height * 5 / 4) as isize),
                    (width / 2) as c_int,
                    width as c_int,
                    height as c_int,
                );
            }
            target
        }
    }
}
