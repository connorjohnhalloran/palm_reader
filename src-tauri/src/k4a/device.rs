use crate::*;
use tauri::{App, Manager};

use super::Result::*;
use super::*;

use core::time;
use std::ops::Range;
use std::result::Result;
use std::sync::mpsc::*;
use std::{ptr, thread};

// TODO: Fix confusion over this and handle()
unsafe impl Send for Device {}
unsafe impl Sync for Device {}
//unsafe impl Deserialize for Device {}
pub struct Device {
    handle: *mut k4a_device_t,
    is_recording: bool,
}

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
impl Device {
    // --------------------------------------------------------------------------------
    // --------------------------------- CONSTRUCTORS ---------------------------------
    // --------------------------------------------------------------------------------

    /// Opens the kinect with the given index.
    pub fn with_index(index: u32) -> Result<Device, String> {
        if Self::get_installed_count() == 0 {
            return Err("No devices available.".into());
        }

        let handle = create_handle::<k4a_device_t>();
        let device = Device {
            handle,
            is_recording: false,
        };

        match Self::open(index, &device) {
            Succeeded => Ok(device),
            Failed => Err("Unable to open device.".into()),
        }
    }

    /// Opens the kinect with the given serial number.
    pub fn with_serialnum(serialnum: &str) -> Result<Device, String> {
        // TODO: Fix this so it won't panic when trying to check when a device is already open
        // It might be alright but it could have issues
        for i in Self::installed() {
            let device = Self::with_index(i)?;
            if serialnum == device.get_serialnum() {
                return Ok(device);
            }
        }
        Err(format!("Device with serial number {} not connected.", serialnum).into())
    }

    // --------------------------------------------------------------------------------
    // ----------------------------------- METHODS ------------------------------------
    // --------------------------------------------------------------------------------

    /*
     */
    pub fn b() {}

    /// Returns the serial number of the kinect.
    pub fn get_serialnum(&self) -> String {
        unsafe {
            // Get length of serial number string (because C)
            let serial_length = create_handle::<usize>();
            k4a_device_get_serialnum(self.handle(), ptr::null_mut::<i8>(), serial_length);

            // Get the actual serial number
            let serial_number = Box::new([0 as i8; 32]);
            k4a_device_get_serialnum(
                self.handle(),
                serial_number.as_ptr() as *mut i8,
                serial_length,
            );

            // Convert null terminated ascii string to a rust string
            serial_number
                .iter()
                .map(|&x| char::from(x as u8))
                .take(*serial_length - 1)
                .collect::<String>()
        }
    }

    pub fn capture_loop(&self, app: &App) -> Result<Sender<()>, String> {
        let app_handle = app.handle();
        let (tx, rx) = channel();

        thread::spawn(move || loop {
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    println!("Terminating.");
                    break;
                }
                Err(TryRecvError::Empty) => {
                    println!("Capture!");
                    app_handle.emit_all("test-event", "Hello Tauri!").unwrap();
                    thread::sleep(time::Duration::from_millis(33));
                }
            }
        });

        Ok(tx)
    }
    /// TODO: Rework to thread spawn setup
    pub fn start_capture(&mut self) -> Result<(), String> {
        if self.is_recording {
            return Err("W Capture is already active.".into());
        }
        if self.start_cameras() == Failed {
            return Err("E Unable to start cameras.".into());
        }
        self.is_recording = true;
        Ok(())
    }

    pub fn stop_capture(&mut self) -> Result<(), String> {
        if !self.is_recording {
            return Err("W Capture is already stopped.".into());
        }
        self.stop_cameras();
        self.is_recording = false;
        Ok(())
    }

    // --------------------------------------------------------------------------------
    // ------------------------------ PRIVATE FUNCTIONS -------------------------------
    // --------------------------------------------------------------------------------

    /// Opens the Kinect device. Called by new().
    fn open(index: u32, device: &Device) -> super::Result {
        unsafe {
            match k4a_device_open(index, device.open_handle()) {
                k4a_result_t_K4A_RESULT_SUCCEEDED => Succeeded,
                k4a_result_t_K4A_RESULT_FAILED | _ => Failed,
            }
        }
    }

    /// Closes the kinect device. Called by drop().
    fn close(device: &mut Device) {
        unsafe { k4a_device_close(device.handle()) }
    }

    /// Provides the handle to the device object.
    pub fn handle(&self) -> *mut _k4a_device_t {
        unsafe { *self.handle }
    }

    /// Provides the handle with an extra layer of indirection for open().
    pub fn open_handle(&self) -> *mut *mut _k4a_device_t {
        self.handle
    }

    /// Returns a range of indexes for cleaner syntax when iterating over devices.
    fn installed() -> Range<u32> {
        0..Self::get_installed_count()
    }
    ///

    fn start_cameras(&self) -> super::Result {
        //It is not valid to call k4a_device_start_cameras() a second time on the
        // same k4a_device_t until k4a_device_stop_cameras() has been called.
        unsafe {
            match k4a_device_start_cameras(self.handle(), &default_config()) {
                k4a_result_t_K4A_RESULT_SUCCEEDED => Succeeded,
                k4a_result_t_K4A_RESULT_FAILED | _ => Failed,
            }
        }
    }

    ///
    fn stop_cameras(&self) {
        unsafe {
            k4a_device_stop_cameras(self.handle());
        }
    }

    // --------------------------------------------------------------------------------
    // ------------------------------- PUBLIC FUNCTIONS -------------------------------
    // --------------------------------------------------------------------------------

    /// Returns the number of available devices.
    pub fn get_installed_count() -> u32 {
        unsafe { k4a_device_get_installed_count() }
    }

    /// Closes the kinect device. Called by drop().
    pub fn get_calibration(&self) -> *mut _k4a_calibration_t {
        let handle = create_handle::<k4a_calibration_t>();
        unsafe {
            k4a_device_get_calibration(
                self.handle(),
                k4a_depth_mode_t_K4A_DEPTH_MODE_NFOV_UNBINNED,
                k4a_color_resolution_t_K4A_COLOR_RESOLUTION_OFF,
                handle,
            );
            handle
        }
    }
}
// TODO: Figure out why closing window doesn't drop
impl Drop for Device {
    fn drop(&mut self) {
        let _ = self.stop_capture();
        Self::close(self);
    }
}
