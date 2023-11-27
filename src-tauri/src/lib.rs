#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use pyo3::prelude::*;
use std::sync::{mpsc::Sender, Arc, Mutex};
use webrtc::{data_channel::RTCDataChannel, peer_connection::RTCPeerConnection};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod k4a;
pub mod models;
pub mod rtc;

#[pyfunction]
fn open_recording() {
    let p = k4a::Playback::open("E:\\palm_reader\\recordings\\test_recording.mkv");
    let c = p.get_next_capture().expect("Error reading capture");
    let image = c.get_depth_image();

    let depth =
        unsafe { core::slice::from_raw_parts(image.buffer() as *const u16, image.size() / 2) };
    let depth_i32 = depth.into_iter().map(|x| *x as i32).collect::<Vec<i32>>();

    println!("{:?}", depth);
}

#[pymodule]
fn palm_reader(_py: Python, m: &PyModule) -> PyResult<()> {
    println!("Hello");
    m.add_function(wrap_pyfunction!(open_recording, m)?)?;
    Ok(())
}

pub struct MyState(
    pub Mutex<Option<Arc<RTCPeerConnection>>>,
    pub Mutex<Option<Arc<RTCDataChannel>>>,
    pub Mutex<Option<Arc<Sender<()>>>>,
);
