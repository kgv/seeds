use crate::utils::SyncMat;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Convex hull
#[derive(Clone, Debug, Deserialize, Hash, Serialize)]
pub struct ConvexHull {
    #[serde(skip)]
    pub points: Arc<SyncMat>,
    pub clockwise: bool,
    pub return_points: bool,
}

impl Default for ConvexHull {
    fn default() -> Self {
        Self {
            points: Default::default(),
            clockwise: false,
            return_points: true,
        }
    }
}
