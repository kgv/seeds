use super::Result;
use crate::{node::GreaterThan, utils::SyncMat};
use egui::util::cache::{ComputerMut, FrameCache};
use opencv::core::{greater_than_mat_f64, MatExprTraitConst};
use std::sync::Arc;

/// Greater than cache
pub type GreaterThanCache = FrameCache<Result<Arc<SyncMat>>, GreaterThanComputer>;

/// Greater than computer
#[derive(Default)]
pub struct GreaterThanComputer {}

impl ComputerMut<&GreaterThan, Result<Arc<SyncMat>>> for GreaterThanComputer {
    fn compute(&mut self, key: &GreaterThan) -> Result<Arc<SyncMat>> {
        Ok(Arc::new(SyncMat(
            greater_than_mat_f64(&*key.a, key.s)?.to_mat()?,
        )))
    }
}
