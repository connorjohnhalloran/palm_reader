use std::mem;

mod capture;
pub use capture::*;

mod device;
pub use device::*;

mod image;
pub use image::*;

mod recording;
pub use recording::*;

mod playback;
pub use playback::*;

use crate::*;

#[derive(PartialEq)]
pub enum Result {
    Succeeded = k4a_result_t_K4A_RESULT_SUCCEEDED as isize,
    Failed = k4a_result_t_K4A_RESULT_FAILED as isize,
}

#[allow(dead_code)]
pub enum BufferResult {
    Succeeded = k4a_buffer_result_t_K4A_BUFFER_RESULT_SUCCEEDED as isize,
    Failed = k4a_buffer_result_t_K4A_BUFFER_RESULT_FAILED as isize,
    TooSmall = k4a_buffer_result_t_K4A_BUFFER_RESULT_TOO_SMALL as isize,
}

pub fn create_handle<T>() -> *mut T {
    unsafe { Box::<T>::into_raw(Box::new(mem::zeroed())) }
}
pub fn default_config() -> k4a_device_configuration_t {
    k4a_device_configuration_t {
        color_format: k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_MJPG,
        color_resolution: k4a_color_resolution_t_K4A_COLOR_RESOLUTION_720P,
        depth_mode: k4a_depth_mode_t_K4A_DEPTH_MODE_NFOV_UNBINNED,
        camera_fps: k4a_fps_t_K4A_FRAMES_PER_SECOND_30,
        synchronized_images_only: true,
        depth_delay_off_color_usec: 0,
        wired_sync_mode: k4a_wired_sync_mode_t_K4A_WIRED_SYNC_MODE_STANDALONE,
        subordinate_delay_off_master_usec: 0,
        disable_streaming_indicator: false,
    }
}

pub fn depth_only_config() -> k4a_device_configuration_t {
    k4a_device_configuration_t {
        color_format: k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_MJPG,
        color_resolution: k4a_color_resolution_t_K4A_COLOR_RESOLUTION_OFF,
        depth_mode: k4a_depth_mode_t_K4A_DEPTH_MODE_NFOV_UNBINNED,
        camera_fps: k4a_fps_t_K4A_FRAMES_PER_SECOND_30,
        synchronized_images_only: true,
        depth_delay_off_color_usec: 0,
        wired_sync_mode: k4a_wired_sync_mode_t_K4A_WIRED_SYNC_MODE_STANDALONE,
        subordinate_delay_off_master_usec: 0,
        disable_streaming_indicator: false,
    }
}
