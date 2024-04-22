use super::Result;
use crate::{node::Read, utils::SyncMat};
use egui::util::cache::{ComputerMut, FrameCache};
use opencv::imgcodecs::{imread, IMREAD_COLOR};
use std::sync::Arc;

/// Read cache
pub type ReadCache = FrameCache<Result<Arc<SyncMat>>, ReadComputer>;

/// Read computer
#[derive(Default)]
pub struct ReadComputer {}

impl ComputerMut<&Read, Result<Arc<SyncMat>>> for ReadComputer {
    fn compute(&mut self, key: &Read) -> Result<Arc<SyncMat>> {
        let filename = &*key.path.to_string_lossy();
        Ok(Arc::new(SyncMat(imread(filename, IMREAD_COLOR)?)))
    }
}
