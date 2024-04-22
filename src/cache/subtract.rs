use super::Result;
use crate::{node::Subtract, utils::SyncMat};
use egui::util::cache::{ComputerMut, FrameCache};
use opencv::core::{subtract_def, Mat};
use std::sync::Arc;

/// Subtract cache
pub type SubtractCache = FrameCache<Result<Arc<SyncMat>>, SubtractComputer>;

/// Subtract computer
#[derive(Default)]
pub struct SubtractComputer {}

impl ComputerMut<&Subtract, Result<Arc<SyncMat>>> for SubtractComputer {
    fn compute(&mut self, key: &Subtract) -> Result<Arc<SyncMat>> {
        let mut dst = Mat::default();
        subtract_def(&*key.src1, &*key.src2, &mut dst)?;
        Ok(Arc::new(SyncMat(dst)))
    }
}
