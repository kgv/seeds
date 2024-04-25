pub(crate) use self::{
    convert_color::ConvertColor, convex_hull::ConvexHull, dilate::Dilate,
    find_contours::FindContours, greater_than::GreaterThan, median_blur::MedianBlur, read::Read,
    subtract::Subtract, threshold::Threshold, write::Write,
};

use crate::utils::SyncMat;
use opencv::core::{type_to_string, Mat, _InputArray};
use opencv::prelude::MatTraitConst;
use opencv::{
    boxed_ref::BoxedRef,
    core::{MatTraitConstManual, ToInputArray},
    Result,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{
    ffi::c_void,
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
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
    pub fn as_mat_mut(&mut self, index: usize) -> &mut Arc<SyncMat> {
        match self {
            Self::Write(Write {
                img: Value::Matrix(matrix),
                ..
            }) => matrix,
            Self::Write(Write {
                img: Value::Matrix(matrix),
                ..
            }) => matrix,
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

/// Value
#[derive(Clone, Debug)]
pub enum Value {
    Matrix(Matrix),
    Vector(Vec<Mat>),
}
impl Default for Value {
    fn default() -> Self {
        Self::Matrix(Default::default())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::Matrix(matrix) => Display::fmt(matrix, f),
            Value::Vector(vector) => f.debug_list().entries(vector.iter().map(Matrix)).finish(),
        }
    }
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::Matrix(matrix) => matrix.hash(state),
            Value::Vector(vector) => {
                for mat in vector {
                    Matrix(mat).hash(state)
                }
            }
        }
    }
}

/// Sync [`Mat`]
#[derive(Clone, Debug, Default)]
#[repr(transparent)]
pub struct Matrix<T = Mat>(pub T);

impl<T: MatTraitConst> Display for Matrix<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Matrix")
            .field(
                "type",
                &type_to_string(self.0.typ()).map_err(|_| fmt::Error)?,
            )
            .field("rows", &self.rows())
            .field("columns", &self.cols())
            .finish()
    }
}

impl Hash for Matrix<Mat> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.data_bytes().ok().hash(state)
    }
}

impl Hash for Matrix<&Mat> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.data_bytes().ok().hash(state)
    }
}

unsafe impl<T> Sync for Matrix<T> {}

impl<T: MatTraitConst> MatTraitConst for Matrix<T> {
    fn as_raw_Mat(&self) -> *const c_void {
        self.0.as_raw_Mat()
    }
}

impl<T: ToInputArray> ToInputArray for Matrix<T> {
    fn input_array(&self) -> Result<BoxedRef<_InputArray>> {
        self.0.input_array()
    }
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
