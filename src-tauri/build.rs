extern crate bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    tauri_build::build();
    gen();
}

fn gen() {
    println!("cargo:rustc-link-search=C:\\Program Files\\Azure Kinect SDK v1.4.1\\sdk\\windows-desktop\\amd64\\release\\bin");
    println!("cargo:rustc-link-search=C:\\Program Files\\Azure Kinect SDK v1.4.1\\sdk\\windows-desktop\\amd64\\release\\lib");
    println!("cargo:rustc-link-lib=k4a");
    println!("cargo:rustc-link-lib=k4arecord");
    //println!("cargo:rustc-link-lib=depthengine_2_0");

    // ---------------------------------------------------------------------
    // IF THE LINKER IS HAVING TROUBLE MAKE SURE TO ADD THE DLLS TO THE PATH
    // ---------------------------------------------------------------------

    let k4a_header = "C:\\Program Files\\Azure Kinect SDK v1.4.1\\sdk\\include\\k4a\\k4a.h";
    let k4arecord_header =
        "C:\\Program Files\\Azure Kinect SDK v1.4.1\\sdk\\include\\k4arecord\\record.h";
    let k4aplayback_header =
        "C:\\Program Files\\Azure Kinect SDK v1.4.1\\sdk\\include\\k4arecord\\playback.h";

    let include_dir = "C:\\Program Files\\Azure Kinect SDK v1.4.1\\sdk\\include";
    let write_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");

    bindgen::Builder::default()
        .header(k4a_header)
        .header(k4arecord_header)
        .header(k4aplayback_header)
        //.clang_arg("-v")
        .clang_arg("-I")
        .clang_arg(include_dir)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Binding generation error")
        .write_to_file(write_dir)
        .expect("Couldn't write bindings!");
}
