use crate::utils::SyncMat;
use egui::epaint::util::FloatOrd;
use serde::{Deserialize, Serialize};
use std::{
    hash::{Hash, Hasher},
    path::PathBuf,
    sync::Arc,
};

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
    pub fn r#in(&mut self, index: usize) -> &mut Arc<SyncMat> {
        match self {
            Self::Write(Write { img, .. }) => img,
            Self::ConvertColor(ConvertColor { src, .. }) => src,
            // Self::ConvexHull(ConvexHull { src, .. }) => src,
            Self::Dilate(Dilate { src, .. }) => src,
            Self::GreaterThan(GreaterThan { a, .. }) => a,
            Self::MedianBlur(MedianBlur { src, .. }) => src,
            Self::Subtract(Subtract { src1, .. }) if index == 0 => src1,
            Self::Subtract(Subtract { src2, .. }) if index == 1 => src2,
            Self::Threshold(Threshold { src, .. }) => src,
            _ => unreachable!(),
        }
    }
}

/// Read
#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct Read {
    pub path: PathBuf,
}

/// Write
#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct Write {
    #[serde(skip)]
    pub img: Arc<SyncMat>,
    pub path: PathBuf,
}

/// Convert color
#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct ConvertColor {
    #[serde(skip)]
    pub src: Arc<SyncMat>,
    pub from: Option<Color>,
    pub to: Option<Color>,
}

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

/// Dilate
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Dilate {
    #[serde(skip)]
    pub src: Arc<SyncMat>,
    pub kernel: Kernel,
    pub anchor: Point,
    pub iterations: i32,
}

impl Default for Dilate {
    fn default() -> Self {
        Self {
            src: Default::default(),
            kernel: Default::default(),
            anchor: Point { x: -1, y: -1 },
            iterations: Default::default(),
        }
    }
}

impl Hash for Dilate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.src.hash(state);
        self.kernel.hash(state);
        self.anchor.hash(state);
        self.iterations.hash(state);
    }
}

/// Find contours
#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct FindContours {
    #[serde(skip)]
    pub image: Arc<SyncMat>,
    pub mode: i32,
    pub method: i32,
    pub offset: Point,
}

/// Greater than
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GreaterThan {
    #[serde(skip)]
    pub a: Arc<SyncMat>,
    pub s: f64,
}

impl Hash for GreaterThan {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.a.hash(state);
        self.s.ord().hash(state);
    }
}

/// Median blur
#[derive(Clone, Debug, Deserialize, Hash, Serialize)]
pub struct MedianBlur {
    #[serde(skip)]
    pub src: Arc<SyncMat>,
    pub ksize: i32,
}

impl Default for MedianBlur {
    fn default() -> Self {
        Self {
            src: Default::default(),
            ksize: 1,
        }
    }
}

/// Subtract
#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct Subtract {
    #[serde(skip)]
    pub src1: Arc<SyncMat>,
    #[serde(skip)]
    pub src2: Arc<SyncMat>,
}

/// Threshold
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Threshold {
    #[serde(skip)]
    pub src: Arc<SyncMat>,
    pub thresh: f64,
    pub maxval: f64,
}

impl Default for Threshold {
    fn default() -> Self {
        Self {
            src: Default::default(),
            thresh: 0.0,
            maxval: 255.0,
        }
    }
}

impl Hash for Threshold {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.src.hash(state);
        self.thresh.ord().hash(state);
        self.maxval.ord().hash(state);
    }
}

/// Color
#[derive(Clone, Copy, Debug, Deserialize, Hash, Serialize)]
pub enum Color {
    Bgr,
    Gray,
}

/// Kernel
#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct Kernel {
    pub rows: i32,
    pub cols: i32,
    pub typ: i32,
}

/// Point
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, Serialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
