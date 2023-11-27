use crate::k4a::*;
use std::{fs, thread};
use tauri::async_runtime::block_on;

/*
pub struct AppState(pub Mutex<Option<RTCPeerConnection>>);
impl AppState {
    pub fn init() -> AppState {
        AppState(Mutex::new(None))
    }
}
*/

#[tauri::command]
pub fn go() {
    thread::spawn(move || {
        let mut device: Device = Device::with_index(0).expect("Error: Unable to open device.");
        device.start_capture();

        for _ in 0..150 {
            println!("In loop");
            let capture = Capture::new();
            capture.get_capture(&device);
            let image = capture.get_color_image();
            println!("{}", image.timestamp());
        }

        device.stop_capture().expect("WHWOH");
    });
}

// --------------------------------------------------------------------------------

/*
#[tauri::command]
pub fn start_capture(state: State<AppState>) -> Result<(), String> {
    (*state.0.lock().unwrap()).start_capture()?;
    Ok(())
}

#[tauri::command]
pub fn stop_capture(state: State<AppState>) -> Result<(), String> {
    (*state.0.lock().unwrap()).stop_capture()?;
    let cap = Capture::new();
    cap.get_color_image();
    Ok(())
}

   RECORDING:

       k4a_record_create(path, device, device_config, recording_handle)
       k4a_record_write_header(recording_handle)

       k4a_capture_create(capture_handle)
       k4a_capture_get_color_image(capture_handle)

       k4a_record_write_capture(recording_handle, capture_handle)

       k4a_record_flush(recording_handle)
       k4a_record_close(recording_handle)
*/
