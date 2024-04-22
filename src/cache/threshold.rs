use super::Result;
use crate::{node::Threshold, utils::SyncMat};
use egui::util::cache::{ComputerMut, FrameCache};
use opencv::{
    core::Mat,
    imgproc::{threshold, THRESH_BINARY_INV, THRESH_OTSU},
};
use std::sync::Arc;

/// Threshold cache
pub type ThresholdCache = FrameCache<Result<Arc<SyncMat>>, ThresholdComputer>;

/// Threshold computer
#[derive(Default)]
pub struct ThresholdComputer {}

impl ComputerMut<&Threshold, Result<Arc<SyncMat>>> for ThresholdComputer {
    fn compute(&mut self, key: &Threshold) -> Result<Arc<SyncMat>> {
        let mut dst = Mat::default();
        threshold(
            &*key.src,
            &mut dst,
            key.thresh,
            key.maxval,
            THRESH_BINARY_INV | THRESH_OTSU,
        )?;
        Ok(Arc::new(SyncMat(dst)))
    }
}
