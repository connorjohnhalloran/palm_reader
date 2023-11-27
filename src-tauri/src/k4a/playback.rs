use std::ffi::CString;

use super::*;
use crate::*;

pub struct Playback {
    handle: *mut k4a_playback_t,
}
impl Playback {
    pub fn new() -> Playback {
        let handle = create_handle::<k4a_playback_t>();
        Playback { handle }
    }

    /// Provides the handle to the device object.
    fn handle(&self) -> *mut _k4a_playback_t {
        unsafe { *self.handle }
    }

    /// Provides the handle with an extra layer of indirection for open().
    fn open_handle(&self) -> *mut *mut _k4a_playback_t {
        self.handle
    }

    pub fn open(path: &str) -> Playback {
        let p = Playback::new();
        unsafe {
            k4a_playback_open(CString::new(path).unwrap().as_ptr(), p.open_handle());
        }
        p
    }

    pub fn get_next_capture(&self) -> Option<Capture> {
        let capture = Capture::new();
        unsafe {
            match k4a_playback_get_next_capture(self.handle(), capture.open_handle()) {
                k4a_stream_result_t_K4A_STREAM_RESULT_SUCCEEDED => Some(capture),
                _ => None,
            }
        }
    }
}
impl Drop for Playback {
    fn drop(&mut self) {
        unsafe {
            k4a_playback_close(self.handle());
        }
    }
}
