use super::Result;
use crate::{node::MedianBlur, utils::SyncMat};
use egui::util::cache::{ComputerMut, FrameCache};
use opencv::{core::Mat, imgproc::median_blur};
use std::sync::Arc;

/// Median blur cache
pub type MedianBlurCache = FrameCache<Result<Arc<SyncMat>>, MedianBlurComputer>;

/// Median blur computer
#[derive(Default)]
pub struct MedianBlurComputer {}

impl ComputerMut<&MedianBlur, Result<Arc<SyncMat>>> for MedianBlurComputer {
    fn compute(&mut self, key: &MedianBlur) -> Result<Arc<SyncMat>> {
        let mut dst = Mat::default();
        median_blur(&*key.src, &mut dst, key.ksize)?;
        Ok(Arc::new(SyncMat(dst)))
    }
}
