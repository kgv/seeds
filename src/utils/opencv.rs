use opencv::core::ToInputArray;
use opencv::imgproc::{cvt_color_def, dilate, distance_transform_def, morphology_ex};
use opencv::{core::Mat, imgcodecs::imwrite_def, imgproc::threshold, Result};
use std::path::Path;

pub trait MatExt {
    fn cvt_color(&self, code: i32) -> Result<Mat>;
    fn dilate(&self, kernel: &impl ToInputArray, iterations: i32) -> Result<Mat>;
    fn distance_transform(&self, distance_type: i32, mask_size: i32) -> Result<Mat>;
    fn morphology(&self, op: i32, kernel: &impl ToInputArray, iterations: i32) -> Result<Mat>;
    fn threshold(&self, tresh: f64, maxval: f64, typ: i32) -> Result<Mat>;
    fn write(&self, path: impl AsRef<Path>) -> Result<()>;
}

impl MatExt for Mat {
    fn cvt_color(&self, code: i32) -> Result<Mat> {
        let mut dst = Mat::default();
        cvt_color_def(self, &mut dst, code)?;
        Ok(dst)
    }

    fn dilate(&self, kernel: &impl ToInputArray, iterations: i32) -> Result<Mat> {
        let mut dst = Mat::default();
        dilate(
            self,
            &mut dst,
            kernel,
            Default::default(),
            iterations,
            Default::default(),
            Default::default(),
        )?;
        Ok(dst)
    }

    fn distance_transform(&self, distance_type: i32, mask_size: i32) -> Result<Mat> {
        let mut dst = Mat::default();
        distance_transform_def(self, &mut dst, distance_type, mask_size)?;
        Ok(dst)
    }

    // * To remove any small white noises in the image, we can use morphological opening.
    // * To remove any small holes in the object, we can use morphological closing.
    fn morphology(&self, op: i32, kernel: &impl ToInputArray, iterations: i32) -> Result<Mat> {
        let mut dst = Mat::default();
        morphology_ex(
            self,
            &mut dst,
            op,
            kernel,
            Default::default(),
            iterations,
            Default::default(),
            Default::default(),
        )?;
        Ok(dst)
    }

    fn threshold(&self, tresh: f64, maxval: f64, typ: i32) -> Result<Mat> {
        let mut dst = Mat::default();
        threshold(self, &mut dst, tresh, maxval, typ)?;
        Ok(dst)
    }

    fn write(&self, path: impl AsRef<Path>) -> Result<()> {
        imwrite_def(path.as_ref().to_str().unwrap(), self)?;
        Ok(())
    }
}
