use super::Result;
use crate::{node::DrawContours, utils::SyncMat};
use egui::util::cache::{ComputerMut, FrameCache};
use opencv::{
    core::{Mat, VecN, Vector},
    imgproc::draw_contours_def,
};
use std::sync::Arc;

/// Draw contours cache
pub type DrawContoursCache = FrameCache<Result<Arc<SyncMat>>, DrawContoursComputer>;

/// Draw contours computer
#[derive(Default)]
pub struct DrawContoursComputer {}

// impl ComputerMut<&DrawContours, Result<Arc<SyncMat>>> for DrawContoursComputer {
//     fn compute(&mut self, key: &DrawContours) -> Result<Arc<SyncMat>> {
//         let mut dst = Mat::default();
//         let src = Vector::<Mat>::from_iter(key.contours);
//         draw_contours_def(&mut dst, &src, -1, VecN(key.color))?;
//         Ok(Arc::new(SyncMat(dst)))
//     }
// }
