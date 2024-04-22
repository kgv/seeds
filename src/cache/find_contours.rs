use super::Result;
use crate::{node::FindContours, utils::SyncMat};
use egui::util::cache::{ComputerMut, FrameCache};
use opencv::{
    core::{Mat, Point, Vector},
    imgproc::find_contours,
};
use std::sync::Arc;

/// Find contours cache
pub type FindContoursCache = FrameCache<Result<Arc<Vec<SyncMat>>>, FindContoursComputer>;

/// Find contours computer
#[derive(Default)]
pub struct FindContoursComputer {}

impl ComputerMut<&FindContours, Result<Arc<Vec<SyncMat>>>> for FindContoursComputer {
    fn compute(&mut self, key: &FindContours) -> Result<Arc<Vec<SyncMat>>> {
        let mut contours = Vector::<Mat>::new();
        find_contours(
            &*key.image,
            &mut contours,
            key.mode,
            key.method,
            Point::new(key.offset.x, key.offset.y),
        )?;
        Ok(Arc::new(contours.into_iter().map(SyncMat).collect()))
    }
}
