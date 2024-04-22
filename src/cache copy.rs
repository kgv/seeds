use crate::{
    node::{
        ConvertColor, Dilate, FindContours, GreaterThan, MedianBlur, Node, Read, Subtract,
        Threshold, Write,
    },
    utils::SyncMat,
};
use anyhow::{anyhow, Result};
use egui::util::cache::{ComputerMut, FrameCache};
use opencv::{
    core::{greater_than_mat_f64, subtract_def, Mat, MatExprTraitConst, Point, Vector, CV_8U},
    imgcodecs::{imread, imwrite_def, IMREAD_COLOR},
    imgproc::{
        cvt_color_def, dilate, find_contours, median_blur, threshold, COLOR_BGR2GRAY,
        THRESH_BINARY_INV, THRESH_OTSU,
    },
};
use std::sync::Arc;
use tracing::error;

/// Node cache
pub type NodeCache = FrameCache<Option<Arc<SyncMat>>, NodeComputer>;

/// Node computer
#[derive(Default)]
pub struct NodeComputer {}

impl NodeComputer {
    fn try_compute(&mut self, key: &Node) -> Result<Arc<SyncMat>> {
        match key {
            Node::Read(Read { path }) => {
                let filename = path.to_str().ok_or(anyhow!("path to str"))?;
                Ok(Arc::new(SyncMat(imread(filename, IMREAD_COLOR)?)))
            }
            Node::Write(Write { img, path }) => {
                let filename = path.to_str().ok_or(anyhow!("path to str"))?;
                imwrite_def(filename, &**img)?;
                Ok(Default::default())
            }
            Node::ConvertColor(ConvertColor { src, from, to }) => {
                let mut dst = Mat::default();
                cvt_color_def(&**src, &mut dst, COLOR_BGR2GRAY)?;
                Ok(Arc::new(SyncMat(dst)))
            }
            // Node::ConvexHull(ConvexHull { src, from, to }) => {
            //     let mut dst = Mat::default();
            //     cvt_color_def(&**src, &mut dst, COLOR_BGR2GRAY)?;
            //     Ok(Arc::new(SyncMat(dst)))
            // }
            Node::Dilate(Dilate {
                src,
                kernel,
                anchor,
                iterations,
            }) => {
                let mut dst = Mat::default();
                dilate(
                    &**src,
                    &mut dst,
                    &Mat::ones(kernel.rows, kernel.cols, CV_8U)?,
                    Point::new(anchor.x, anchor.y),
                    *iterations,
                    Default::default(),
                    Default::default(),
                )?;
                Ok(Arc::new(SyncMat(dst)))
            }
            Node::FindContours(FindContours {
                image,
                mode,
                method,
                offset,
            }) => {
                let mut contours = Vector::<Mat>::new();
                find_contours(
                    &**image,
                    &mut contours,
                    *mode,
                    *method,
                    Point::new(offset.x, offset.y),
                )?;
                Ok(Arc::new(SyncMat(contours)))
            }
            Node::GreaterThan(GreaterThan { a, s }) => {
                Ok(Arc::new(SyncMat(greater_than_mat_f64(&**a, *s)?.to_mat()?)))
            }
            Node::MedianBlur(MedianBlur { src, ksize }) => {
                let mut dst = Mat::default();
                median_blur(&**src, &mut dst, *ksize)?;
                Ok(Arc::new(SyncMat(dst)))
            }
            Node::Subtract(Subtract { src1, src2 }) => {
                let mut dst = Mat::default();
                subtract_def(&**src1, &**src2, &mut dst)?;
                Ok(Arc::new(SyncMat(dst)))
            }
            Node::Threshold(Threshold {
                src,
                thresh,
                maxval,
            }) => {
                tracing::warn!("Threshold");
                let mut out = Mat::default();
                threshold(
                    &**src,
                    &mut out,
                    *thresh,
                    *maxval,
                    THRESH_BINARY_INV | THRESH_OTSU,
                )?;
                Ok(Arc::new(SyncMat(out)))
            }
        }
    }
}

impl ComputerMut<&Node, Option<Arc<SyncMat>>> for NodeComputer {
    fn compute(&mut self, key: &Node) -> Option<Arc<SyncMat>> {
        match self.try_compute(key) {
            Ok(value) => Some(value),
            Err(error) => {
                error!(%error);
                None
            }
        }
    }
}

// impl ComputerMut<&Node, Option<Arc<SyncMat>>> for NodeComputer {
//     fn compute(&mut self, key: &Node) -> Option<Arc<SyncMat>> {
//         match key {
//             Node::Read(Read { path }) => {
//                 let filename = path.to_str()?;
//                 Some(Arc::new(SyncMat(imread(filename, IMREAD_COLOR).ok()?)))
//             }
//             Node::Write(Write { img, path }) => {
//                 let filename = path.to_str()?;
//                 tracing::error!("Write: {filename}, {img:?}");
//                 imwrite_def(filename, &**img).ok()?;
//                 None
//             }
//             Node::ConvertColor(ConvertColor { src, from, to }) => {
//                 let mut dst = Mat::default();
//                 cvt_color_def(&**src, &mut dst, COLOR_BGR2GRAY).ok()?;
//                 Some(Arc::new(SyncMat(dst)))
//             }
//             Node::GreaterThan(GreaterThan { a, s }) => Some(Arc::new(SyncMat(
//                 greater_than_mat_f64(&**a, *s).ok()?.to_mat().ok()?,
//             ))),
//             Node::MedianBlur(MedianBlur { src, ksize }) => {
//                 let mut dst = Mat::default();
//                 median_blur(&**src, &mut dst, *ksize).ok()?;
//                 Some(Arc::new(SyncMat(dst)))
//             }
//             Node::Subtract(Subtract { src1, src2 }) => {
//                 let mut dst = Mat::default();
//                 subtract_def(&**src1, &**src2, &mut dst).ok()?;
//                 Some(Arc::new(SyncMat(dst)))
//             }
//             Node::Threshold(Threshold {
//                 src,
//                 thresh,
//                 maxval,
//             }) => {
//                 let mut out = Mat::default();
//                 threshold(
//                     &**src,
//                     &mut out,
//                     *thresh,
//                     *maxval,
//                     THRESH_BINARY_INV | THRESH_OTSU,
//                 )
//                 .ok()?;
//                 Some(Arc::new(SyncMat(out)))
//             }
//         }
//     }
// }
