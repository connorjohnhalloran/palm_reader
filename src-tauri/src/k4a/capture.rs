use crate::*;

use super::*;

pub struct Capture {
    handle: *mut k4a_capture_t,
}

#[allow(dead_code)]
impl Capture {
    pub fn new() -> Capture {
        let handle = create_handle::<k4a_capture_t>();
        let capture = Capture { handle };
        capture.create();
        capture
    }

    pub fn create(&self) {
        unsafe {
            k4a_capture_create(self.open_handle());
        }
    }
    pub fn get_capture(&self, device: &Device) {
        unsafe {
            k4a_device_get_capture(device.handle(), self.handle, 3000);
        }
    }

    pub fn get_color_image(&self) -> Image {
        unsafe { Image::from(k4a_capture_get_color_image(self.handle())) }
    }

    pub fn get_depth_image(&self) -> Image {
        unsafe { Image::from(k4a_capture_get_depth_image(self.handle())) }
    }

    /// Provides the handle to the device object.
    pub fn handle(&self) -> *mut _k4a_capture_t {
        unsafe { *self.handle }
    }

    /// Provides the handle with an extra layer of indirection for open().
    pub fn open_handle(&self) -> *mut *mut _k4a_capture_t {
        self.handle
    }
}

impl Drop for Capture {
    fn drop(&mut self) {
        unsafe {
            k4a_capture_release(self.handle());
        }
    }
}
