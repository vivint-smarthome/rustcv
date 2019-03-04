extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

#[cfg(unix)]
fn opencv_link() {
    let link_search = env::var("OPENCV_LIB_DIR").unwrap_or("/usr/local/bin".to_owned());
    println!("cargo:rustc-link-search=all={}", link_search);
    println!("cargo:rustc-link-lib=opencv_core");
    println!("cargo:rustc-link-lib=opencv_dnn");
    println!("cargo:rustc-link-lib=opencv_features2d");
    println!("cargo:rustc-link-lib=opencv_highgui");
    println!("cargo:rustc-link-lib=opencv_imgcodecs");
    println!("cargo:rustc-link-lib=opencv_imgproc");
    println!("cargo:rustc-link-lib=opencv_objdetect");
    println!("cargo:rustc-link-lib=opencv_video");
    println!("cargo:rustc-link-lib=opencv_videoio");
    if cfg!(feature = "text") {
        println!("cargo:rustc-link-lib=opencv_text");
    }
    if cfg!(feature = "contrib") {
        println!("cargo:rustc-link-lib=opencv_xfeatures2d");
    }
    if cfg!(feature = "cuda") {
        println!("cargo:rustc-link-lib=opencv_cudaobjdetect");
    }
}

fn source(module: &str) -> String {
    let mut path = String::from("gocv/");
    path += module;
    path += ".cpp";
    path
}

fn generate_binding() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindgen::builder()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("opencv-sys.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    generate_binding();

    let modules = vec![
        "core",
        "dnn",
        "features2d",
        "highgui",
        "imgcodecs",
        "imgproc",
        "objdetect",
        "version",
        "video",
        "videoio",
    ];

    let mut sources: Vec<String> = modules.into_iter().map(source).collect();

    if cfg!(feature = "cuda") {
        sources.push("cuda.cpp".to_string());
    }

    let mut builder = cc::Build::new();
    builder.flag("-std=c++11")
        .warnings(false)
        .cpp(true)
        .files(sources);
    env::var("OPENCV_INCLUDE_DIR").ok().map(|dir| {
        eprintln!("Including dir {}", dir);
        builder.include(dir)
    });
    builder.compile("cv");

    opencv_link();
}
