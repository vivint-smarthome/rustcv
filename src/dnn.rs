//! [Deep Neural Network](https://docs.opencv.org/master/d6/d0f/group__dnn.html).

use opencv_sys as ffi;
use core::{Mat, Scalar, Size, Mats};
use std::ffi::CString;
use std::rc::*;
use Error;

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
        let mut bytes:Vec<u8> = Vec::new();
        for s in output_names_ {
            let mut v = s.as_bytes_with_nul().to_vec();
            bytes.append(&mut v);
        }
        let names = ffi::CStrings{strs: bytes.as_mut_ptr() as *mut *const u8, length: total_length as i32};


        let mut mats_:Vec<Mat> = Vec::with_capacity(total_length);
        let mut mats = ffi::Mats{mats: mats_.as_mut_ptr() as *mut *mut ::std::os::raw::c_void, length: total_length as i32};
        //let mut mats_ptr : *mut ffi::Mats = &mats;
        unsafe{ffi::Net_ForwardLayers(self.inner, &mut mats, names)};
        //ffi::Net_ForwardLayers(self.inner, mats_ptr, names);
        Ok(Mats {
            inner: mats
        })
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

//pub fn get_mat(blob: ffi::Mats, index: i32) -> (ffi::Mats, Mat) {
//    let rc_mats = Rc::new(blob);
//    let get_mats = rc_mats.clone();
//    let mat = ffi::Mats_get(get_mats, index);
//    (rc_mats.try_unwrap().unwrap(), mat)
//}

//   /// Get the names of the output layers
//   fn get_outputs_names(net : Net) -> Vec<String> {
//       //  IntVector {
//       //      val: *mut ::std::os::raw::c_int,
//       //      length: ::std::os::raw::c_int,
//       //      };
//       //fn Net_GetUnconnectedOutLayers(net: Net, res: *mut IntVector);
//       // pub fn Net_GetLayer(net: Net, layerid: ::std::os::raw::c_int) -> Layer;
//       // pub fn Layer_OutputNameToIndex(
//       // fn Layer_GetName(layer: Layer) -> *const ::std::os::raw::c_char;
//       //let names : Vec<String> = Vec::new();
//       //let out_layers : Vec<i32> = Vec::new();
//       let out_layers : IntVector = IntVector{val: *mut ::std::os::raw::c_int, length: ::std::os::raw::c_int,};
//       ffi::Net_GetUnconnectedOutLayers(net, &mut out_layers);
//       let names = out_layers.iter().map(|i| {
//         let layer = ffi::Net_GetLayer(net, i);
//         let name = ffi::Layer_GetName(layer);
//         name
//       });
//       names
//   
//   }

