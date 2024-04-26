use super::Result;
use crate::node::{Value, Write};
use egui::util::cache::{ComputerMut, FrameCache};
use finder::RED;
use opencv::core::Mat;
use opencv::core::Vector;
use opencv::imgcodecs::imwrite_def;
use opencv::imgproc::draw_contours_def;

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
