// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bytes::Bytes;
use palm_reader::k4a::*;
use palm_reader::models::*;
use palm_reader::rtc::*;
use palm_reader::MyState;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::slice::from_raw_parts;
use std::sync::mpsc::*;
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::async_runtime::block_on;
use tauri::State;
use webrtc::data_channel::RTCDataChannel;
use webrtc::peer_connection::RTCPeerConnection;

fn main() {
    tauri::Builder::default()
        .manage(MyState(
            Default::default(),
            Default::default(),
            Default::default(),
        ))
        .invoke_handler(tauri::generate_handler![
            create_offer,
            remote_ready,
            register_ice,
            start_capture,
            stop_capture,
            start_recording,
            stop_recording,
            open_recording,
            edit_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    println!("After tauri setup.");
}

#[tauri::command]
fn edit_data(path: String) {
    println!("Opening {}", path);

    let mut device: Device = Device::with_index(0).expect("Error: Unable to open device.");
    let playback = Playback::open(&path);
    let mut i = 1;

    loop {
        let c = playback.get_next_capture();

        match c {
            None => break,
            _ => (),
        }

        let c = c.unwrap();

        let depth = c.get_depth_image();
        println!("{}", depth.width());
        let xyz = depth.pointcloud(&device);
        println!("WHHAHAHAH");
        let points =
            unsafe { from_raw_parts(xyz.buffer() as *const (i16, i16, i16), xyz.size() / 6) };

        let mut w = Vec::new();
        for p in points {
            if !(p.0 == 0 && p.1 == 0 && p.2 == 0) {
                w.push(format!("v {} {} {}\n", p.0, p.1, p.2));
            }
        }

        let p = format!("E:\\palm_reader\\hou\\capture_{}.obj", i);

        thread::spawn(move || {
            let mut file = File::create(p).unwrap();
            file.write_all(b"g\n").unwrap();
            w.iter().for_each(|s| file.write_all(s.as_bytes()).unwrap());
        });
        i += 1;
    }

    //k4a_transformation_depth_image_to_point_cloud

    let out =
        Command::new("C:/Program Files/Side Effects Software/Houdini 20.0.506/bin/hython.exe")
            .arg("E:/palm_reader/hou/test.py")
            .arg("E:/palm_reader/ml/openclosed/data/test1")
            .arg(path)
            .output()
            .expect("error running hython");
    println!("{}", String::from_utf8(out.stdout).unwrap());
    println!("{}", String::from_utf8(out.stderr).unwrap());
}

#[tauri::command]
fn start_capture(state: State<'_, MyState>) {
    //let pc = (*(state.0.lock().unwrap())).as_ref().unwrap().clone();
    let dc = (*(state.1.lock().unwrap())).as_ref().unwrap().clone();

    let v1 = V1::new();

    let mut device: Device = Device::with_index(0).expect("Error: Unable to open device.");
    device.start_capture().expect("Error starting capture");
    let (tx, rx) = channel::<()>();
    *(state.2.lock().unwrap()) = Some(Arc::new(tx));

    thread::spawn(move || {
        loop {
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }
            let capture = Capture::new();
            capture.get_capture(&device);

            let image = capture.get_depth_image();

            let depth = unsafe { from_raw_parts(image.buffer() as *const u16, image.size() / 2) };
            let depth_i32 = depth.into_iter().map(|x| *x as i32).collect::<Vec<i32>>();

            //println!("{:?}", depth_i32);
            //println!("{}", image.timestamp());
            //println!("{}", image.size());
            //println!("{}", depth.len());
            //println!("{}", image.format());

            let prediction = v1.eval(depth_i32);

            let message = &Bytes::from(prediction);
            let r = dc.send(message);
            let _ = block_on(r).map_err(|_err| {
                println!("ERROR SENDING MESSAGE");
            });
        }
        device.stop_capture().expect("WHWOH");
    });
}

#[tauri::command]
fn stop_capture(state: State<'_, MyState>) {
    let _ = (*(state.2.lock().unwrap())).as_ref().unwrap().send(());
}

#[tauri::command]
fn start_recording(state: State<'_, MyState>) {
    let mut device: Device = Device::with_index(0).expect("Error: Unable to open device.");
    device.start_capture().expect("Error starting capture");

    let rec = Recording::new(&device, "E:\\palm_reader\\recordings\\test_recording.mkv");

    //rec.write_header();

    let (tx, rx) = channel::<()>();
    *(state.2.lock().unwrap()) = Some(Arc::new(tx));

    // Need to get a handle to stop this from the
    thread::spawn(move || {
        loop {
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    // TODO: Add finish recording code here
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }
            let capture = Capture::new();
            capture.get_capture(&device);

            rec.write_capture(&capture);
        }
        device.stop_capture().expect("WHWOH");
    });
}

#[tauri::command]
fn stop_recording(state: State<'_, MyState>) {
    let _ = (*(state.2.lock().unwrap())).as_ref().unwrap().send(());
}

#[tauri::command]
fn open_recording(state: State<'_, MyState>) {
    let p = Playback::open("E:\\palm_reader\\recordings\\test_recording.mkv");
    let c = p.get_next_capture().unwrap();
    let image = c.get_depth_image();

    let depth = unsafe { from_raw_parts(image.buffer() as *const u16, image.size() / 2) };
    let depth_i32 = depth.into_iter().map(|x| *x as i32).collect::<Vec<i32>>();

    println!("{:?}", depth);
}
