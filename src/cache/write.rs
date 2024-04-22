use super::Result;
use crate::node::Write;
use egui::util::cache::{ComputerMut, FrameCache};
use opencv::imgcodecs::imwrite_def;

/// Write cache
pub type WriteCache = FrameCache<Result<()>, WriteComputer>;

/// Write computer
#[derive(Default)]
pub struct WriteComputer {}

impl ComputerMut<&Write, Result<()>> for WriteComputer {
    fn compute(&mut self, key: &Write) -> Result<()> {
        let filename = &*key.path.to_string_lossy();
        imwrite_def(filename, &*key.img)?;
        Ok(())
    }
}
