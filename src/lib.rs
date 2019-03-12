//! This library primarily provides a binding and API for OpenCV 3.x.
//!
//! The wrapper is a direct steal from
//! [GoCV](https://github.com/hybridgroup/gocv).

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]

use failure::*;

use failure::Error;
use std::path::{Path, PathBuf};
use std::ffi::CString;

pub mod core;
#[cfg(feature = "dnn")]
pub mod dnn;
#[cfg(feature = "features2d")]
pub mod features2d;
#[cfg(feature = "highgui")]
pub mod highgui;
#[cfg(feature = "imgcodecs")]
pub mod imgcodecs;
#[cfg(feature = "imgproc")]
pub mod imgproc;
#[cfg(feature = "objedetect")]
pub mod objdetect;
#[cfg(feature = "cuda")]
pub mod cuda;

#[derive(Debug, Fail)]
/// Custom errors.
pub enum CvError {
    #[fail(display = "invalid string: {:?}", _0)]
    /// Indicates that string was invalid
    InvalidString(String),

    #[fail(display = "invalid path: {:?}", _0)]
    /// Indicates that path was invalid
    InvalidPath(PathBuf),

    #[fail(display = "error loading cascade: {:?}", _0)]
    /// Indicates that cascade model was invalid
    InvalidCascadeModel(PathBuf),

    #[fail(display = "EntryNotFound: {:?}", _0)]
    /// Indicates that there is no entry on specified path
    EntryNotFound(PathBuf),
    #[fail(display = "failed to convert from primitive: {}", value)]
    /// Indicates that conversion from primitive to enum type is failed
    EnumFromPrimitiveConversionError {
        /// Value that caused an error
        value: i32,
    },
    #[fail(display = "Unknown error: {:?}", _0)]
    /// Indicates that error occurred in C++ code
    UnknownError(String),
    #[fail(display = "Non ascii characters found in string: {:?}", _0)]
    /// Indicates that string contains non ascii characters
    UnicodeChars(String),
}

fn path_to_cstring<P: AsRef<Path>>(path: P) -> Result<CString, Error> {
    let path = path.as_ref();
    let x = path.to_str().ok_or_else(|| CvError::InvalidPath(path.into()))?;
    let result = CString::new(x)?;
    Ok(result)
}
