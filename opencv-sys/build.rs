use std::env;
use std::path::PathBuf;

const OPENCV_LIB_DIR: &str = "OPENCV_LIB_DIR";
const OPENCV_INCLUDE_DIR: &str= "OPENCV_INCLUDE_DIR";

#[cfg(unix)]
fn opencv_link() {
    let link_static = if cfg!(feature = "build-opencv") {
        "=static"
    } else {
        ""
    };
    let link_search = env::var(OPENCV_LIB_DIR).unwrap_or("/usr/local/bin".to_owned());
    println!("cargo:rustc-link-search=all={}", link_search);
    println!("cargo:rustc-link-lib{}=opencv_core", link_static);
    println!("cargo:rustc-link-lib{}=opencv_dnn", link_static);
    println!("cargo:rustc-link-lib{}=opencv_features2d", link_static);
    println!("cargo:rustc-link-lib{}=opencv_highgui", link_static);
    println!("cargo:rustc-link-lib{}=opencv_imgcodecs", link_static);
    println!("cargo:rustc-link-lib{}=opencv_imgproc", link_static);
    println!("cargo:rustc-link-lib{}=opencv_objdetect", link_static);
    println!("cargo:rustc-link-lib{}=opencv_video", link_static);
    println!("cargo:rustc-link-lib{}=opencv_videoio", link_static);
    if cfg!(feature = "text") {
        println!("cargo:rustc-link-lib{}=opencv_text", link_static);
    }
    if cfg!(feature = "contrib") {
        println!("cargo:rustc-link-lib{}=opencv_xfeatures2d", link_static);
    }
    if cfg!(feature = "cuda") {
        println!("cargo:rustc-link-lib{}=opencv_cudaobjdetect", link_static);
    }
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

fn build_opencv() {
    #[cfg(feature="build-opencv")]
    {
        use std::collections::HashMap;
        let mut config = cmake::Config::new("opencv");
        let mut defines = HashMap::<String, String>::new();
        {
            let mut define = |k: &str, v: &str| defines.insert(k.into(), v.into());
            define("INSTALL_C_EXAMPLES", "OFF");
            define("BUILD_EXAMPLES", "OFF");
            define("BUILD_PERF_TESTS", "OFF");
            define("BUILD_TESTS", "OFF");
            define("BUILD_DOCS", "OFF");
            // Statically link the libraries
            define("BUILD_SHARED_LIBS", "OFF");
            define("BUILD_opencv_python_bindings_generator", "OFF");
            define("BUILD_opencv_java_bindings_generator", "OFF");
            define("BUILD_opencv_stitching", "ON");
            define("BUILD_opencv_photo", "ON");
            define("BUILD_opencv_flann", "ON");
            define("BUILD_opencv_highgui", "ON");
            define("BUILD_opencv_video", "ON");
            define("BUILD_opencv_calib3d", "ON");
            define("BUILD_opencv_shape", "ON");
            define("BUILD_opencv_objdetect", "ON");
            define("BUILD_opencv_ml", "ON");
            define("BUILD_opencv_core", "ON");
            define("BUILD_opencv_dnn", "ON");
            define("BUILD_opencv_features2d", "ON");
        }
        let manifest_dir= env::var("CARGO_MANIFEST_DIR").expect("Cargo should provide manifest directory!");
        let opencv_dir = manifest_dir + "/opencv";
        env::vars().for_each(|(k, v)|{
            const DEFINE: &str = "RUSTCV_OPENCV_DEFINE_";
            const ENV: &str = "RUSTCV_OPENCV_ENV_";
            let v = v.replace("RUSTCV_OPENCV_GIT_DIR", &opencv_dir);
            if k.starts_with(DEFINE) {
                println!("cargo:rerun-if-env-changed={}", &k);
                let k = k.replace(DEFINE, "");
                defines.insert(k, v);
            } else if k.starts_with(ENV) {
                println!("cargo:rerun-if-env-changed={}", &k);
                let k = k.replace(ENV, "");
                config.env(k, v);
            }
        });
        defines.into_iter().for_each(|(k, v)|{
            eprintln!("Defining {}={}", &k, &v);
            config.define(k, v);
        });
        config.env("MAKEFLAGS", env::var("MAKEFLAGS").unwrap_or_else(|_|format!("-j{}", num_cpus::get() - 1)));
        let dst = config.very_verbose(false).build();
        env::set_var(OPENCV_LIB_DIR, dst.join("lib"));
        env::set_var(OPENCV_INCLUDE_DIR, dst.join("include"));
    }
}

fn main() {
    println!("cargo:rerun-if-env-changed={}", OPENCV_LIB_DIR);
    println!("cargo:rerun-if-env-changed={}", OPENCV_INCLUDE_DIR);
    build_opencv();
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

    let mut sources: Vec<String> = modules.into_iter().map(|m|format!("gocv/{}.cpp", m)).collect();

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
