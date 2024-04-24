pub(crate) use self::{
    convert_color::ConvertColor, convex_hull::ConvexHull, dilate::Dilate,
    find_contours::FindContours, greater_than::GreaterThan, median_blur::MedianBlur, read::Read,
    subtract::Subtract, threshold::Threshold, write::Write,
};

use crate::utils::SyncMat;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Node
#[derive(Clone, Debug, Deserialize, Hash, Serialize)]
pub enum Node {
    // codecs
    Read(Read),
    Write(Write),
    // proc
    ConvertColor(ConvertColor),
    // ConvexHull(ConvexHull),
    Dilate(Dilate),
    FindContours(FindContours),
    GreaterThan(GreaterThan),
    MedianBlur(MedianBlur),
    Subtract(Subtract),
    Threshold(Threshold),
}

impl Node {
    pub fn mat_mut(&mut self, index: usize) -> &mut Arc<SyncMat> {
        match self {
            Self::Write(Write { img, .. }) => img,
            Self::ConvertColor(ConvertColor { src, .. }) => src,
            // Self::ConvexHull(ConvexHull { src, .. }) => src,
            Self::Dilate(Dilate { src, .. }) => src,
            Self::FindContours(FindContours { image, .. }) => image,
            Self::GreaterThan(GreaterThan { a, .. }) => a,
            Self::MedianBlur(MedianBlur { src, .. }) => src,
            Self::Subtract(Subtract { src1, .. }) if index == 0 => src1,
            Self::Subtract(Subtract { src2, .. }) if index == 1 => src2,
            Self::Threshold(Threshold { src, .. }) => src,
            _ => unreachable!(),
        }
    }
}

/// Color
#[derive(Clone, Copy, Debug, Deserialize, Hash, Serialize)]
pub enum Color {
    Bgr,
    Gray,
}

/// Point
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, Serialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

mod convert_color;
mod convex_hull;
mod dilate;
mod find_contours;
mod greater_than;
mod median_blur;
mod read;
mod subtract;
mod threshold;
mod write;
