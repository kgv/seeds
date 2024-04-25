use super::Result;
use crate::node::{Value, Write};
use egui::util::cache::{ComputerMut, FrameCache};
use opencv::core::Mat;
use opencv::core::Scalar;
use opencv::core::Vector;
use opencv::imgcodecs::imwrite_def;
use opencv::imgproc::draw_contours_def;

const RED: Scalar = Scalar::new(0.0, 0.0, 255.0, 0.0);

/// Write cache
pub type WriteCache = FrameCache<Result<()>, WriteComputer>;

/// Write computer
#[derive(Default)]
pub struct WriteComputer {}

impl ComputerMut<&Write, Result<()>> for WriteComputer {
    fn compute(&mut self, key: &Write) -> Result<()> {
        let filename = &*key.path.to_string_lossy();
        match &key.img {
            Value::Matrix(matrix) => {
                imwrite_def(filename, &matrix.0)?;
            }
            Value::Vector(vector) => {
                let mut image = Mat::default();
                let contours = Vector::from_iter(vector);
                for (index, contour) in vector.iter().enumerate() {
                    draw_contours_def(&mut image, &contours, index as _, RED)?;
                }
                imwrite_def(filename, &image)?;
            }
        }
        Ok(())
    }
}
