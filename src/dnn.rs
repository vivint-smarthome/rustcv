//! [Deep Neural Network](https://docs.opencv.org/master/d6/d0f/group__dnn.html).

use opencv_sys as ffi;
use core::{Mat, Scalar, Size, Mats};
use std::ffi::CString;
use Error;

/// Backends available for use by DNN
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub enum DnnBackend {
    /// Default
    DnnBackendDefault,
    /// Halide gpu
    DnnBackendHalide,
    /// Inference engine
    DnnBackendInferenceEngine,
    /// opencv optimized
    DnnBackendOpencv,
}


/// Targets available for use with DNN
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub enum DnnTarget {
    /// On cpu
    DnnTargetCpu,
    /// gpu using opencl
    DnnTargetOpencl,
    /// gpu using opencl with fp16
    DnnTargetOpenclFp16,
    /// myriad gpu
    DnnTargetMyriad,
}

/// Cascade classifier class for object detection.
#[derive(Debug)]
pub struct Net {
    inner: ffi::Net,
}

impl Net {

    /// Reads a network model, supported models are:
    /// * Caffe (caffemodel, prototxt)
    /// * Tensorflow (pb, pbtxt)
    /// * Darknet (weights, cfg)
    /// * OpenVino (bin, xml)
    /// * Torch (t7)
    pub fn read_net(model: &str, config: &str) -> Result<Self, Error> {
        let model = CString::new(model)?;
        let config= CString::new(config)?;

        Ok(Net {
            inner: unsafe { ffi::Net_ReadNet(model.as_ptr(), config.as_ptr()) },
        })
    }

    /// Reads a network model stored in Caffe framework's format.
    pub fn from_caffe(prototxt: &str, model: &str) -> Result<Self, Error> {
        let prototxt = CString::new(prototxt)?;
        let model = CString::new(model)?;

        Ok(Net {
            inner: unsafe { ffi::Net_ReadNetFromCaffe(prototxt.as_ptr(), model.as_ptr()) },
        })
    }

    /// Reads a network model stored in Caffe framework's format.
    pub fn from_tensorflow(model: &str) -> Result<Self, Error> {
        let model = CString::new(model)?;
        Ok(Net {
            inner: unsafe { ffi::Net_ReadNetFromTensorflow(model.as_ptr()) },
        })
    }

    /// Returns true if there are no layers in the network.
    pub fn empty(&self) -> bool {
        unsafe { ffi::Net_Empty(self.inner) }
    }

    /// Sets the new value for the layer output blob.
    pub fn set_input(&mut self, blob: &Mat, name: &str) -> Result<(), Error> {
        let name = CString::new(name)?;
        unsafe { ffi::Net_SetInput(self.inner, blob.inner, name.as_ptr()) };
        Ok(())
    }

    /// Runs forward pass to compute output of layer with name outputName.
    pub fn forward(&self, output_name: &str) -> Result<Mat, Error> {
        let output_name = CString::new(output_name)?;
        Ok(Mat::from(unsafe {
            ffi::Net_Forward(self.inner, output_name.as_ptr())
        }))
    }

    /// Run forward pass to compute output of layers with outputNames
    // TODO refactor - this is ugly!
    pub fn forward_multi(&self, output_names: Vec<String>) -> Result<Mats, Error> {
        let mut output_names_: Vec<CString> = vec![];
        let total_length = output_names.len();
        for s in output_names.into_iter() {
            output_names_.push(CString::new(s).unwrap());
        }
        let mut output_names2: Vec<*const u8> = output_names_.iter().map(|s|{s.as_ptr()}).collect();
        let names = ffi::CStrings{strs: output_names2.as_mut_ptr(), length: total_length as i32};

        let mut mats_: Vec<*mut ::std::os::raw::c_void> = Vec::with_capacity(total_length);
        let mut mats = ffi::Mats{mats: mats_.as_mut_ptr(), length: total_length as i32};
        //let mut mats_ptr : *mut ffi::Mats = &mats;
        unsafe{ffi::Net_ForwardLayers(self.inner, &mut mats, names)};
        //ffi::Net_ForwardLayers(self.inner, mats_ptr, names);
        Ok(Mats {
            inner: mats
        })
    }

    /// Set the desired backend
    pub fn set_preferable_backend(&self, backend: DnnBackend) {
        let bkend: ::std::os::raw::c_int = match  backend {
            DnnBackend::DnnBackendDefault => 0,
            DnnBackend::DnnBackendHalide => 1,
            DnnBackend::DnnBackendInferenceEngine => 2,
            DnnBackend::DnnBackendOpencv => 3,
        };
        unsafe {
            ffi::Net_SetPreferableBackend(self.inner, bkend);
        }

    }

    /// Set the target for the computation
    pub fn set_preferable_target(&self, target: DnnTarget) {
        let trgt: ::std::os::raw::c_int = match target {
            DnnTarget::DnnTargetCpu => 0,
            DnnTarget::DnnTargetMyriad => 3,
            DnnTarget::DnnTargetOpencl => 1,
            DnnTarget::DnnTargetOpenclFp16 => 2,
        };
        unsafe {
            ffi::Net_SetPreferableTarget(self.inner, trgt);
        }

    }

}

/// Creates 4-dimensional blob from image. Optionally resizes and crops image
/// from center, subtract mean values, scales values by scalefactor, swap Blue
/// and Red channels.
pub fn blob_from_image(
    img: &Mat,
    scale: f64,
    size: Size,
    mean: Scalar,
    swap_rb: bool,
    crop: bool,
) -> Mat {
    Mat::from(unsafe { ffi::Net_BlobFromImage(img.inner, scale, size, mean, swap_rb, crop) })
}

impl Drop for Net {
    fn drop(&mut self) {
        unsafe { ffi::Net_Close(self.inner) }
    }
}

/// Extracts a single (2d)channel from a 4 dimensional blob structure (this
///  might e.g. contain the results of a SSD or YOLO detection, a bones
///  structure from pose detection, or a color plane from Colorization)
pub fn get_blob_channel(blob: &Mat, image_index: i32, channel_index: i32) -> Mat {
    Mat::from(unsafe { ffi::Net_GetBlobChannel(blob.inner, image_index, channel_index) })
}

/// Retrieves the 4 dimensional size information in (N,C,H,W) order
pub fn get_blob_size(blob: &Mat) -> Scalar {
    unsafe { ffi::Net_GetBlobSize(blob.inner) }
}


