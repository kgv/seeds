use super::Result;
use crate::{node::ConvertColor, utils::SyncMat};
use egui::util::cache::{ComputerMut, FrameCache};
use opencv::{
    core::Mat,
    imgproc::{cvt_color_def, COLOR_BGR2GRAY},
};
use std::sync::Arc;

/// Convert color cache
pub type ConvertColorCache = FrameCache<Result<Arc<SyncMat>>, ConvertColorComputer>;

/// Convert color computer
#[derive(Default)]
pub struct ConvertColorComputer {}

impl ComputerMut<&ConvertColor, Result<Arc<SyncMat>>> for ConvertColorComputer {
    fn compute(&mut self, key: &ConvertColor) -> Result<Arc<SyncMat>> {
        let mut dst = Mat::default();
        cvt_color_def(&*key.src, &mut dst, COLOR_BGR2GRAY)?;
        Ok(Arc::new(SyncMat(dst)))
    }
}
