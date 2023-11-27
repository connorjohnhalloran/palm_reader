use std::ffi::CString;

use super::*;
use crate::*;

unsafe impl Send for Recording {}
pub struct Recording {
    handle: *mut k4a_record_t,
}

#[allow(temporary_cstring_as_ptr)]
#[allow(dead_code)]
impl Recording {
    pub fn new(device: &Device, path: &str) -> Recording {
        let handle = create_handle::<k4a_record_t>();
        let record = Recording { handle };

        record.create(&device, path);
        record.write_header();

        record
    }

    fn create(&self, device: &Device, path: &str) {
        unsafe {
            k4a_record_create(
                CString::new(path).unwrap().as_ptr(),
                device.handle(),
                depth_only_config(),
                self.open_handle(),
            );
        }
    }

    pub fn write_header(&self) {
        unsafe {
            k4a_record_write_header(self.handle());
        }
    }

    pub fn write_capture(&self, capture: &Capture) {
        unsafe {
            k4a_record_write_capture(self.handle(), capture.handle());
        }
    }

    pub fn flush(&self) {
        unsafe {
            k4a_record_flush(self.handle());
        }
    }

    /// Provides the handle to the device object.
    fn handle(&self) -> *mut _k4a_record_t {
        unsafe { *self.handle }
    }

    /// Provides the handle with an extra layer of indirection for open().
    fn open_handle(&self) -> *mut *mut _k4a_record_t {
        self.handle
    }
}
impl Drop for Recording {
    fn drop(&mut self) {
        unsafe {
            k4a_record_close(self.handle());
        }
    }
}
