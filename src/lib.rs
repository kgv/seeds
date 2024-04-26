pub use self::config::Config;

use anyhow::Result;
use opencv::{
    core::{Mat, MatTraitConst, Scalar, VecN},
    imgproc::{cvt_color_def, COLOR_HSV2BGR},
};
use serde::{Deserialize, Serialize};
use std::iter::once;

pub const BLACK: Scalar = Scalar::all(0.0);
pub const WHITE: Scalar = Scalar::all(255.0);
pub const BLUE: Scalar = Scalar::new(255.0, 0.0, 0.0, 0.0);
pub const CYAN: Scalar = Scalar::new(255.0, 255.0, 0.0, 0.0);
pub const GREEN: Scalar = Scalar::new(0.0, 255.0, 0.0, 0.0);
pub const YELLOW: Scalar = Scalar::new(0.0, 255.0, 255.0, 0.0);
pub const RED: Scalar = Scalar::new(0.0, 0.0, 255.0, 0.0);
pub const MAGENTA: Scalar = Scalar::new(255.0, 0.0, 255.0, 0.0);

/// Hsb
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Hsb {
    pub hue: f64,
    pub saturation: f64,
    pub brightness: f64,
}

impl Hsb {
    pub fn bgr(&self) -> Result<VecN<u8, 3>> {
        let src = Mat::from_exact_iter(once(VecN([
            self.hue as u8,
            self.saturation as u8,
            self.brightness as u8,
        ])))?;
        let mut dst = Mat::default();
        cvt_color_def(&src, &mut dst, COLOR_HSV2BGR)?;
        Ok(*dst.at(0)?)
    }
}

pub mod config;
