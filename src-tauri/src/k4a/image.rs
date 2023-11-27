use crate::*;

use super::{create_handle, Device};

pub struct Image {
    handle: *mut k4a_image_t,
}
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
impl Image {
    pub fn new() -> Image {
        let handle = create_handle::<k4a_image_t>();
        let image = Image { handle };
        image.create();
        image
    }
    pub fn new_xyz() -> Image {
        let handle = create_handle::<k4a_image_t>();
        let image = Image { handle };
        image.create_xyz();
        image
    }
    pub fn from(image: k4a_image_t) -> Image {
        Image {
            handle: Box::<*mut _k4a_image_t>::into_raw(Box::new(image)),
        }
    }

    pub fn width(&self) -> i32 {
        unsafe { k4a_image_get_width_pixels(self.handle()) }
    }

    pub fn height(&self) -> i32 {
        unsafe { k4a_image_get_height_pixels(self.handle()) }
    }

    pub fn buffer(&self) -> *const u8 {
        unsafe { k4a_image_get_buffer(self.handle()) }
    }

    pub fn pointcloud(&self, device: &Device) -> Image {
        let xyz = Self::new_xyz();
        //let handle = create_handle::<k4a_transformation_t>();
        println!("Inside PC");
        println!("{}", self.width());
        println!("{}", xyz.width());
        unsafe {
            let c = device.get_calibration();
            let t = k4a_transformation_create(c);
            k4a_transformation_depth_image_to_point_cloud(
                t,
                self.handle(),
                k4a_calibration_type_t_K4A_CALIBRATION_TYPE_DEPTH,
                xyz.handle(),
            );
        }
        xyz
    }

    pub fn format(&self) -> k4a_image_format_t {
        unsafe { k4a_image_get_format(self.handle()) }
    }

    pub fn size(&self) -> usize {
        unsafe { k4a_image_get_size(self.handle()) }
    }

    pub fn timestamp(&self) -> u64 {
        unsafe { k4a_image_get_timestamp_usec(self.handle()) }
    }

    fn create(&self) {
        unsafe {
            let r = k4a_image_create(
                k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_BGRA32,
                1920,
                1080,
                0,
                self.open_handle(),
            );

            match r {
                k4a_result_t_K4A_RESULT_SUCCEEDED => println!("Img success"),
                k4a_result_t_K4A_RESULT_FAILED | _ => println!("IMAGE FAIL :("),
            }
        }
    }

    fn create_xyz(&self) {
        unsafe {
            let r = k4a_image_create(
                k4a_image_format_t_K4A_IMAGE_FORMAT_CUSTOM,
                640,
                576,
                6 * 640,
                self.open_handle(),
            );

            match r {
                k4a_result_t_K4A_RESULT_SUCCEEDED => println!("Img success"),
                k4a_result_t_K4A_RESULT_FAILED | _ => println!("IMAGE FAIL :("),
            }
        }
    }

    /// Provides the handle to the device object.
    fn handle(&self) -> *mut _k4a_image_t {
        unsafe { *self.handle }
    }

    /// Provides the handle with an extra layer of indirection for open().
    fn open_handle(&self) -> *mut *mut _k4a_image_t {
        self.handle
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            k4a_image_release(self.handle());
        }
    }
}
