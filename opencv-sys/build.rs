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
        use std::collections::HashMap;
        let mut config = cmake::Config::new("opencv");
        let mut defines = HashMap::<String, String>::new();
        {
            let mut d = |k: &str, v: &str| defines.insert(k.into(), v.into());
            static ON: &str = "ON";
            static OFF: &str = "OFF";
            d("BUILD_ZLIB", ON);
            d("WITH_PNG", OFF);
            d("BUILD_PROTOBUF", OFF);
            d("WITH_PROTOBUF", OFF);
            d("BUILD_TBB", OFF);
            d("WITH_TBB", OFF);
            d("WITH_1394", OFF);
            d("WITH_OPENGL", OFF);
            d("WITH_OPENCL", OFF);
            d("WITH_V4L", OFF);
            d("WITH_LIBV4L", OFF);
            d("WITH_GTK", OFF);
            d("WITH_GDAL", OFF);
            d("WITH_XINE", OFF);
            d("WITH_FFMPEG", OFF);
            d("BUILD_opencv_cudabgsegm", OFF);
            d("BUILD_opencv_cudalegacy", OFF);
            d("BUILD_opencv_cudafilters", OFF);
            d("BUILD_opencv_cudastereo", OFF);
            d("BUILD_opencv_cudafeatures2d", OFF);
            d("BUILD_opencv_cudaoptflow", OFF);
            d("BUILD_opencv_cudacodec", OFF);
            d("BUILD_opencv_cudaimgproc", OFF);
            d("BUILD_opencv_cudawarping", OFF);
            d("BUILD_opencv_cudaarithm", OFF);
            d("BUILD_opencv_cudaobjdetect", OFF);
            d("BUILD_opencv_cudev", OFF);
            d("BUILD_opencv_superres", OFF);
            d("BUILD_opencv_ts", OFF);
            d("BUILD_opencv_videostab", OFF);
            d("BUILD_opencv_gapi", OFF);
            d("BUILD_opencv_apps", OFF);
            d("BUILD_opencv_world", OFF);
            d("INSTALL_C_EXAMPLES", OFF);
            d("BUILD_EXAMPLES", OFF);
            d("BUILD_PERF_TESTS", OFF);
            d("BUILD_TESTS", OFF);
            d("BUILD_DOCS", OFF);
            d("BUILD_opencv_python_bindings_generator", OFF);
            d("BUILD_opencv_java_bindings_generator", OFF);
            d("BUILD_opencv_stitching", OFF);
            d("BUILD_opencv_photo", OFF);
            d("BUILD_opencv_flann", OFF);
            d("BUILD_opencv_video", OFF);
            d("BUILD_opencv_videoio", OFF);
            d("BUILD_opencv_calib3d", OFF);
            d("BUILD_opencv_shape", OFF);
            d("BUILD_opencv_ml", OFF);
            d("BUILD_JAVA", OFF);
            d("BUILD_ITT", OFF);
            d("BUILD_PACKAGE", OFF);
            d("CPACK_BINARY_DEB", OFF);
            d("CPACK_BINARY_FREEBSD", OFF);
            d("CPACK_BINARY_IFW", OFF);
            d("CPACK_BINARY_NSIS", OFF);
            d("CPACK_BINARY_RPM", OFF);
            d("CPACK_BINARY_STGZ", OFF);
            d("CPACK_BINARY_TBZ2", OFF);
            d("CPACK_BINARY_TGZ", OFF);
            d("CPACK_BINARY_TXZ", OFF);
            d("CPACK_BINARY_TZ", OFF);
            d("CPACK_SOURCE_RPM", OFF);
            d("CPACK_SOURCE_TBZ2", OFF);
            d("CPACK_SOURCE_TGZ", OFF);
            d("CPACK_SOURCE_TXZ", OFF);
            d("CPACK_SOURCE_TZ", OFF);
            d("CPACK_SOURCE_ZIP", OFF);
            d("WITH_CUDA", OFF);
            d("WITH_GSTREAMER", OFF);
            d("WITH_GTK", OFF);
            d("WITH_IMGCODEC_SUNRASTER", OFF);
            d("WITH_IPP", OFF);
            d("WITH_ITT", OFF);
            d("WITH_JASPER", OFF);
            d("WITH_OPENEXR", OFF);
            d("WITH_PTHREADS_PF", OFF);
            d("WITH_QUIRC", OFF);
            d("WITH_TIFF", OFF);
            d("WITH_V4L", OFF);
            d("WITH_VTK", OFF);
            d("WITH_WEBP", OFF);
            d("ccitt", OFF);
            d("logluv", OFF);
            d("lzw", OFF);
            d("mdi", OFF);
            d("next", OFF);
            d("old_jpeg", OFF);
            d("opencv_dnn_PERF_CAFFE", OFF);
            d("opencv_dnn_PERF_CLCAFFE", OFF);
            d("packbits", OFF);
            d("thunder", OFF);

            // Default these to off. They get turned on based on features.
            d("BUILD_opencv_imgproc", OFF);
            d("BUILD_opencv_imgcodecs", OFF);
            d("BUILD_opencv_highgui", OFF);
            d("BUILD_opencv_objdetect", OFF);
            d("BUILD_opencv_dnn", OFF);
            d("BUILD_opencv_features2d", OFF);

            if cfg!(feature = "imgproc") {
                d("BUILD_opencv_imgproc", ON);
            }
            if cfg!(feature = "imgcodecs") {
                d("BUILD_opencv_imgcodecs", ON);
            }
            if cfg!(feature = "highgui") {
                d("BUILD_opencv_highgui", ON);
            }
            if cfg!(feature = "objdetect") {
                d("BUILD_opencv_objdetect", ON);
            }
            if cfg!(feature = "dnn") {
                d("BUILD_opencv_dnn", ON);
                d("BUILD_PROTOBUF", ON);
                d("WITH_PROTOBUF", ON);
                d("OPENCV_DNN_OPENCL", OFF);
            }
            if cfg!(feature = "features2d") {
                d("BUILD_opencv_features2d", ON);
            }
            if cfg!(feature = "cuda") {
                d("BUILD_opencv_cudaobjdetect", ON);
            }
            d("BUILD_opencv_core", ON);
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
        //Profile needs to be Release otherwise you'll end up with some very slow execution
        config.profile("Release");
        let dst = config.build();
        env::set_var(OPENCV_LIB_DIR, dst.join("lib"));
        env::set_var(OPENCV_INCLUDE_DIR, dst.join("include"));
        let lib_3rdparty = dst.join("share/OpenCV/3rdparty/lib");
        env::set_var(format!("{}_3RDPARTY", OPENCV_LIB_DIR), &lib_3rdparty);
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