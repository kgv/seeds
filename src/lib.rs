pub use self::config::Config;

use anyhow::Result;
use opencv::{
    core::{DataType, Mat, MatTraitConst, Scalar, VecN},
    imgproc::{cvt_color_def, COLOR_HSV2BGR},
};
use serde::{Deserialize, Serialize};
use std::iter::once;

pub const BLACK: Scalar = Scalar::all(0.0);
pub const WHITE: Scalar = Scalar::all(255.0);
pub const BLUE: Scalar = Scalar::new(255.0, 0.0, 0.0, 255.0);
pub const CYAN: Scalar = Scalar::new(255.0, 255.0, 0.0, 255.0);
pub const GREEN: Scalar = Scalar::new(0.0, 255.0, 0.0, 255.0);
pub const YELLOW: Scalar = Scalar::new(0.0, 255.0, 255.0, 255.0);
pub const RED: Scalar = Scalar::new(0.0, 0.0, 255.0, 255.0);
pub const MAGENTA: Scalar = Scalar::new(255.0, 0.0, 255.0, 255.0);

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

pub fn hsva_to_bgra(hsva: VecN<f64, 4>) -> Result<VecN<f64, 4>> {
    let src = Mat::from_exact_iter(once(VecN([
        hsva[0].round() as u8,
        hsva[1].round() as _,
        hsva[2].round() as _,
    ])))?;
    let mut dst = Mat::default();
    cvt_color_def(&src, &mut dst, COLOR_HSV2BGR)?;
    let bgr = dst.at::<VecN<u8, 3>>(0)?;
    Ok(VecN([bgr[0] as _, bgr[1] as _, bgr[2] as _, hsva[3]]))
}

impl From<VecN<f64, 3>> for Hsb {
    fn from(value: VecN<f64, 3>) -> Self {
        Self {
            hue: value[0],
            saturation: value[1],
            brightness: value[2],
        }
    }
}

impl From<VecN<f64, 4>> for Hsb {
    fn from(value: VecN<f64, 4>) -> Self {
        Self {
            hue: value[0],
            saturation: value[1],
            brightness: value[2],
        }
    }
}

pub mod config;
pub mod utils;

#[cfg(test)]
mod test {
    use super::*;
    use palette::{cast::from_array, Hsv, Srgb};

    #[test]
    fn test() {
        // [49.997, 120.62, 20.734] [11.0, 20.0, 14.0]
        let source = [49.997, 120.618, 20.734];
        let hsva = from_array::<Hsv<Srgb, f64>>(source).into_format::<u8>();
        println!("{hsva:?}");
        // let rgba = Rgba::<Srgb, f64>::from_color(hsva);
        // let t = means.to::<u8>().unwrap();
        // let hsva = from_array::<Hsva<Srgb, u8>>(*t).into_format::<f64, f64>();
        // let rgba = Rgba::<Srgb, f64>::from_color(hsva);
        // // let hsva = try_from_component_slice::<Hsv<Srgb, u8>>(&t[..4])?;
        // // let t: Srgba = <Srgb>::from_color(hsva);
        // // let h = Hsva<_, u8>::from_components(t);
        // let bgr = Bgra::pack(rgba).into();
        // // println!("{bgr:?}");
        // let bgr = contour.bgr()?.into();
        // [49.997, 120.62, 20.734] [11.0, 20.0, 14.0]
    }
}
