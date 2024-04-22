use super::Result;
use crate::{node::Dilate, utils::SyncMat};
use egui::util::cache::{ComputerMut, FrameCache};
use opencv::{
    core::{Mat, Point, CV_8U},
    imgproc::dilate,
};
use std::sync::Arc;

/// Dilate cache
pub type DilateCache = FrameCache<Result<Arc<SyncMat>>, DilateComputer>;

/// Dilate computer
#[derive(Default)]
pub struct DilateComputer {}

impl ComputerMut<&Dilate, Result<Arc<SyncMat>>> for DilateComputer {
    fn compute(&mut self, key: &Dilate) -> Result<Arc<SyncMat>> {
        let mut dst = Mat::default();
        dilate(
            &*key.src,
            &mut dst,
            &Mat::ones(key.kernel.rows, key.kernel.cols, CV_8U)?,
            Point::new(key.anchor.x, key.anchor.y),
            key.iterations,
            Default::default(),
            Default::default(),
        )?;
        Ok(Arc::new(SyncMat(dst)))
    }
}
