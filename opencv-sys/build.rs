#![cfg_attr(not(test), deny(warnings))]
#![deny(unsafe_code)]

use std::env;
use std::fs;
use std::iter::once;
use std::path::{Path, PathBuf};

static OPENCV_LIB_DIR: &str = "OPENCV_LIB_DIR";
static OPENCV_INCLUDE_DIR: &str = "OPENCV_INCLUDE_DIR";

#[cfg(unix)]
fn opencv_link() {

    fn link_all_in_directory(lib_dir: &str) -> Result<(), std::io::Error> {
        use std::os::unix::ffi::OsStrExt;
        fs::read_dir(&lib_dir)?
            .filter_map(|de| de.ok())
            .filter(|de| de.file_name().as_bytes().starts_with(b"lib"))
            .filter(|de| de.path().is_file())
            .for_each(|de| {
                let name = de.file_name();
                let f = name.to_string_lossy();
                if f.ends_with(".so") {
                    println!("cargo:rustc-link-lib={}", &f[3..f.len() - 3]);
                } else if f.ends_with(".a") {
                    println!("cargo:rustc-link-lib=static={}", &f[3..f.len() - 2]);
                }
            });
        Ok(())
    }

    for (k, lib_dir) in env::vars().filter(|(k, _)| k.starts_with(OPENCV_LIB_DIR)) {
        println!("cargo:rustc-link-search=native={}", &lib_dir);
        println!("cargo:rerun-if-env-changed={}", k);
        link_all_in_directory(&lib_dir)
            .unwrap_or_else(|e| eprintln!("Unable to read dir {}! {}", &lib_dir, e));
    }
    //    println!("cargo:rustc-link-lib{}=opencv_dnn", link_static);
    //    println!("cargo:rustc-link-lib{}=opencv_objdetect", link_static);
    //    println!("cargo:rustc-link-lib{}=opencv_features2d", link_static);
    //    println!("cargo:rustc-link-lib{}=opencv_highgui", link_static);
    //    println!("cargo:rustc-link-lib{}=opencv_imgcodecs", link_static);
    //    println!("cargo:rustc-link-lib{}=opencv_imgproc", link_static);
    //    println!("cargo:rustc-link-lib{}=opencv_core", link_static);
    //    println!("cargo:rustc-link-lib{}=tbb", link_static);
    //    println!("cargo:rustc-link-lib{}=IlmImf", link_static);
    //    println!("cargo:rustc-link-lib{}=libjasper", link_static);
    //    println!("cargo:rustc-link-lib{}=libjpeg-turbo", link_static);
    //    println!("cargo:rustc-link-lib{}=libwebp", link_static);
    //    println!("cargo:rustc-link-lib{}=libpng", link_static);
    //    println!("cargo:rustc-link-lib{}=libprotobuf", link_static);
    //    println!("cargo:rustc-link-lib{}=libtiff", link_static);
    //    println!("cargo:rustc-link-lib{}=tegra_hal", link_static);
    //    println!("cargo:rustc-link-lib{}=zlib", link_static);
}

fn generate_binding<P: AsRef<Path>>(out_dir: P, modules: &[&str]) {
    let mut builder = bindgen::builder();

    'modules: for m in modules.iter().chain(once(&"version")) {
        let paths = vec![
            format!("gocv/{}.h", m),
            format!("gocv/{}_gocv.h", m),
            format!("{}.h", m),
        ];
        'paths: for path in paths {
            if Path::new(&path).exists() {
                println!("cargo:rerun-if-changed={}", path);
                builder = builder.header(path);
                break 'paths;
            }
        }
    }

    builder
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_dir.as_ref().join("opencv-sys.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(unix)]
fn build_opencv<P: AsRef<Path>>(_out_dir: P) {
    #[cfg(feature = "build-opencv")]
    {
        use std::os::unix::ffi::OsStrExt;
        use std::collections::HashMap;
        let mut config = cmake::Config::new("opencv");
        let mut defines = HashMap::<String, String>::new();
        {
            let mut define = |k: &str, v: &str| defines.insert(k.into(), v.into());
            static ON: &str = "ON";
            static OFF: &str = "OFF";
            define("BUILD_ZLIB", ON);
            define("WITH_PNG", OFF);

            define("BUILD_PROTOBUF", OFF);
            define("WITH_PROTOBUF", OFF);
            define("BUILD_TBB", OFF);
            define("WITH_TBB", OFF);
            define("WITH_1394", OFF);
            define("WITH_OPENGL", OFF);
            define("WITH_OPENCL", OFF);
            define("WITH_V4L", OFF);
            define("WITH_LIBV4L", OFF);
            define("WITH_GTK", OFF);
            define("WITH_GDAL", OFF);
            define("WITH_XINE", OFF);
            define("WITH_FFMPEG", OFF);
            define("BUILD_opencv_cudabgsegm", OFF);
            define("BUILD_opencv_cudalegacy", OFF);
            define("BUILD_opencv_cudafilters", OFF);
            define("BUILD_opencv_cudastereo", OFF);
            define("BUILD_opencv_cudafeatures2d", OFF);
            define("BUILD_opencv_cudaoptflow", OFF);
            define("BUILD_opencv_cudacodec", OFF);
            define("BUILD_opencv_cudaimgproc", OFF);
            define("BUILD_opencv_cudawarping", OFF);
            define("BUILD_opencv_cudaarithm", OFF);
            define("BUILD_opencv_cudaobjdetect", OFF);
            define("BUILD_opencv_cudev", OFF);
            define("BUILD_opencv_superres", OFF);
            define("BUILD_opencv_ts", OFF);
            define("BUILD_opencv_videostab", OFF);
            define("BUILD_opencv_gapi", OFF);
            define("BUILD_opencv_apps", OFF);
            define("BUILD_opencv_world", OFF);
            define("INSTALL_C_EXAMPLES", OFF);
            define("BUILD_EXAMPLES", OFF);
            define("BUILD_PERF_TESTS", OFF);
            define("BUILD_TESTS", OFF);
            define("BUILD_DOCS", OFF);
            define("BUILD_opencv_python_bindings_generator", OFF);
            define("BUILD_opencv_java_bindings_generator", OFF);
            define("BUILD_opencv_stitching", OFF);
            define("BUILD_opencv_photo", OFF);
            define("BUILD_opencv_flann", OFF);
            define("BUILD_opencv_video", OFF);
            define("BUILD_opencv_videoio", OFF);
            define("BUILD_opencv_calib3d", OFF);
            define("BUILD_opencv_shape", OFF);
            define("BUILD_opencv_ml", OFF);
            define("BUILD_JAVA", OFF);
            define("BUILD_IPP_IW", OFF);
            define("BUILD_ITT", OFF);
            define("BUILD_PACKAGE", OFF);
            define("CPACK_BINARY_DEB", OFF);
            define("CPACK_BINARY_FREEBSD", OFF);
            define("CPACK_BINARY_IFW", OFF);
            define("CPACK_BINARY_NSIS", OFF);
            define("CPACK_BINARY_RPM", OFF);
            define("CPACK_BINARY_STGZ", OFF);
            define("CPACK_BINARY_TBZ2", OFF);
            define("CPACK_BINARY_TGZ", OFF);
            define("CPACK_BINARY_TXZ", OFF);
            define("CPACK_BINARY_TZ", OFF);
            define("CPACK_SOURCE_RPM", OFF);
            define("CPACK_SOURCE_TBZ2", OFF);
            define("CPACK_SOURCE_TGZ", OFF);
            define("CPACK_SOURCE_TXZ", OFF);
            define("CPACK_SOURCE_TZ", OFF);
            define("CPACK_SOURCE_ZIP", OFF);
            define("WITH_CUDA", OFF);
            define("WITH_GSTREAMER", OFF);
            define("WITH_GTK", OFF);
            define("WITH_IMGCODEC_SUNRASTER", OFF);
            define("WITH_IPP", OFF);
            define("WITH_ITT", OFF);
            define("WITH_JASPER", OFF);
            define("WITH_OPENEXR", OFF);
            define("WITH_PTHREADS_PF", OFF);
            define("WITH_QUIRC", OFF);
            define("WITH_TIFF", OFF);
            define("WITH_V4L", OFF);
            define("WITH_VTK", OFF);
            define("WITH_WEBP", OFF);
            define("ccitt", OFF);
            define("logluv", OFF);
            define("lzw", OFF);
            define("mdi", OFF);
            define("next", OFF);
            define("old_jpeg", OFF);
            define("opencv_dnn_PERF_CAFFE", OFF);
            define("opencv_dnn_PERF_CLCAFFE", OFF);
            define("packbits", OFF);
            define("thunder", OFF);

            // Default these to off. They get turned on based on features.
            define("BUILD_opencv_imgproc", OFF);
            define("BUILD_opencv_imgcodecs", OFF);
            define("BUILD_opencv_highgui", OFF);
            define("BUILD_opencv_objdetect", OFF);
            define("BUILD_opencv_dnn", OFF);
            define("BUILD_opencv_features2d", OFF);

            if cfg!(feature = "imgproc") {
                define("BUILD_opencv_imgproc", ON);
            }
            if cfg!(feature = "imgcodecs") {
                define("BUILD_opencv_imgcodecs", ON);
            }
            if cfg!(feature = "highgui") {
                define("BUILD_opencv_highgui", ON);
            }
            if cfg!(feature = "objdetect") {
                define("BUILD_opencv_objdetect", ON);
            }
            if cfg!(feature = "dnn") {
                define("BUILD_opencv_dnn", ON);
                define("BUILD_PROTOBUF", ON);
                define("WITH_PROTOBUF", ON);
                define("OPENCV_DNN_OPENCL", OFF);
            }
            if cfg!(feature = "features2d") {
                define("BUILD_opencv_features2d", ON);
            }
            if cfg!(feature = "cuda") {
                define("BUILD_opencv_cudaobjdetect", ON);
            }
            define("BUILD_opencv_core", ON);
        }
        let manifest_dir =
            env::var("CARGO_MANIFEST_DIR").expect("Cargo should provide manifest directory!");
        let opencv_dir = manifest_dir + "/opencv";
        static DEFINE: &str = "RUSTCV_OPENCV_DEFINE_";
        static ENV: &str = "RUSTCV_OPENCV_ENV_";
        let target = env::var("TARGET")
            .expect("Cargo should provide TARGET")
            .replace("-", "_")
            .to_uppercase();
        let define_target = format!("{}{}_", DEFINE, &target);
        let env_target = format!("{}{}_", ENV, &target);
        env::vars().for_each(|(k, v)| {
            let v = v.replace("RUSTCV_OPENCV_GIT_DIR", &opencv_dir);
            if k.starts_with(&define_target) {
                println!("cargo:rerun-if-env-changed={}", &k);
                let k = k.replace(&define_target, "");
                defines.insert(k, v);
            } else if k.starts_with(DEFINE) {
                println!("cargo:rerun-if-env-changed={}", &k);
                let k = k.replace(DEFINE, "");
                defines.insert(k, v);
            } else if k.starts_with(&env_target) {
                println!("cargo:rerun-if-env-changed={}", &k);
                let k = k.replace(&env_target, "");
                config.env(k, v);
            } else if k.starts_with(ENV) {
                println!("cargo:rerun-if-env-changed={}", &k);
                let k = k.replace(ENV, "");
                config.env(k, v);
            }
        });
        // Statically link the libraries and override whatever may have been passed in.
        defines.insert("BUILD_SHARED_LIBS".into(), "OFF".into());
        defines.into_iter().for_each(|(k, v)| {
            eprintln!("Defining {}={}", &k, &v);
            config.define(k, v);
        });
        let install_dir = _out_dir.as_ref().join("opencv");
        fs::create_dir_all(&install_dir).expect("Unable to create opencv dir in OUT_DIR");
        config.out_dir(&install_dir);
        let dst = config.very_verbose(false).build();
        env::set_var(OPENCV_LIB_DIR, dst.join("lib"));
        env::set_var(OPENCV_INCLUDE_DIR, dst.join("include"));
        let lib_3rdparty = dst.join("share/OpenCV/3rdparty/lib");
        env::set_var(format!("{}_3RDPARTY", OPENCV_LIB_DIR), &lib_3rdparty);
        fs::read_dir(&lib_3rdparty)
            .expect(&format!(
                "Unable to open 3rdparty lib {}",
                lib_3rdparty.to_string_lossy()
            ))
            .filter_map(|de| de.ok())
            .filter(|de| de.file_name().as_bytes().starts_with(b"liblib"))
            .for_each(|de| {
                let liblib = de.path();
                let liblib_name = liblib
                    .file_name()
                    .expect(&format!(
                        "{} should have a filename!",
                        liblib.to_string_lossy()
                    ))
                    .to_string_lossy();
                let lib_name = &liblib_name[3..];
                let lib = liblib
                    .parent()
                    .expect(&format!(
                        "{} should have a parent!",
                        liblib.to_string_lossy()
                    ))
                    .join(&lib_name);
                fs::rename(&liblib, &lib).expect(&format!(
                    "Unable to rename {} to {}",
                    liblib.to_string_lossy(),
                    lib.to_string_lossy()
                ));
            })
    }
}

#[cfg(unix)]
fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut modules = Vec::with_capacity(10);
    modules.push("core");
    if cfg!(feature = "dnn") {
        modules.push("dnn");
    }
    if cfg!(feature = "features2d") {
        modules.push("features2d");
    }
    if cfg!(feature = "highgui") {
        modules.push("highgui");
    }
    if cfg!(feature = "imgcodecs") {
        modules.push("imgcodecs");
    }
    if cfg!(feature = "imgproc") {
        modules.push("imgproc");
    }
    if cfg!(feature = "objdetect") {
        modules.push("objdetect");
    }
    if cfg!(feature = "cuda") {
        modules.push("cuda");
    }

    generate_binding(&out_dir, &modules);
    build_opencv(&out_dir);

    let sources: Vec<String> = modules
        .into_iter()
        .map(|m| {
            for file in vec![format!("gocv/{}.cpp", m), format!("{}.cpp", m)] {
                if Path::new(&file).exists() {
                    println!("cargo:rerun-if-changed={}", &file);
                    return file;
                }
            }
            panic!("Unable to find .cpp file for {}", m);
        })
        .collect();

    let mut builder = cc::Build::new();
    builder
        .flag("-std=c++11")
        .warnings(false)
        .cpp(true)
        .files(sources);
    env::var(OPENCV_INCLUDE_DIR).ok().map(|dir| {
        println!("cargo:rerun-if-env-changed={}", OPENCV_INCLUDE_DIR);
        eprintln!("Including dir {}", dir);
        builder.include(dir)
    });
    builder.compile("cv");

    opencv_link();
}

#[cfg(not(unix))]
fn main() {
    unimplemented!("This hasn't been implemented for non-*nix platforms yet!");
}