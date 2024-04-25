use crate::{
    utils::SyncMat,
    view::{View, RED, UNTYPED_COLOR},
};
use egui::{ComboBox, Ui};
use egui_snarl::{ui::PinInfo, InPin};
use opencv::imgproc::ColorConversionCodes::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Convert color
#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct ConvertColor {
    #[serde(skip)]
    pub src: Arc<SyncMat>,
    pub code: i32,
}

impl View for ConvertColor {
    fn show_input(&mut self, ui: &mut Ui, pin: &InPin) -> PinInfo {
        match pin.id.input {
            0 if pin.remotes.is_empty() => PinInfo::square().with_fill(UNTYPED_COLOR),
            0 => {
                ui.label(self.src.to_string());
                PinInfo::square().with_fill(RED)
            }
            _ => unreachable!("ConvertColor node has only 1 input"),
        }
    }

    fn show_body(&mut self, ui: &mut Ui) {
        // Code
        ui.horizontal(|ui| {
            ui.label("Code:");
            #[rustfmt::skip]
            ComboBox::from_id_source("code")
                .selected_text(self.code.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.code, COLOR_BGR2BGRA as _, "BGR2BGRA");
                    ui.selectable_value(&mut self.code, COLOR_BGRA2BGR as _, "BGRA2BGR");
                    ui.selectable_value(&mut self.code, COLOR_BGR2RGBA as _, "BGR2RGBA");
                    ui.selectable_value(&mut self.code, COLOR_RGBA2BGR as _, "RGBA2BGR");
                    ui.selectable_value(&mut self.code, COLOR_BGR2RGB as _, "BGR2RGB");
                    ui.selectable_value(&mut self.code, COLOR_BGRA2RGBA as _, "BGRA2RGBA");
                    ui.selectable_value(&mut self.code, COLOR_BGR2GRAY as _, "BGR2GRAY");
                    ui.selectable_value(&mut self.code, COLOR_RGB2GRAY as _, "RGB2GRAY");
                    ui.selectable_value(&mut self.code, COLOR_GRAY2BGR as _, "GRAY2BGR");
                    ui.selectable_value(&mut self.code, COLOR_GRAY2BGRA as _, "GRAY2BGRA");
                    ui.selectable_value(&mut self.code, COLOR_BGRA2GRAY as _, "BGRA2GRAY");
                    ui.selectable_value(&mut self.code, COLOR_RGBA2GRAY as _, "RGBA2GRAY");
                    ui.selectable_value(&mut self.code, COLOR_BGR2BGR565 as _, "BGR2BGR565");
                    ui.selectable_value(&mut self.code, COLOR_RGB2BGR565 as _, "RGB2BGR565");
                    ui.selectable_value(&mut self.code, COLOR_BGR5652BGR as _, "BGR5652BGR");
                    ui.selectable_value(&mut self.code, COLOR_BGR5652RGB as _, "BGR5652RGB");
                    ui.selectable_value(&mut self.code, COLOR_BGRA2BGR565 as _, "BGRA2BGR565");
                    ui.selectable_value(&mut self.code, COLOR_RGBA2BGR565 as _, "RGBA2BGR565");
                    ui.selectable_value(&mut self.code, COLOR_BGR5652BGRA as _, "BGR5652BGRA");
                    ui.selectable_value(&mut self.code, COLOR_BGR5652RGBA as _, "BGR5652RGBA");
                    ui.selectable_value(&mut self.code, COLOR_GRAY2BGR565 as _, "GRAY2BGR565");
                    ui.selectable_value(&mut self.code, COLOR_BGR5652GRAY as _, "BGR5652GRAY");
                    ui.selectable_value(&mut self.code, COLOR_BGR2BGR555 as _, "BGR2BGR555");
                    ui.selectable_value(&mut self.code, COLOR_RGB2BGR555 as _, "RGB2BGR555");
                    ui.selectable_value(&mut self.code, COLOR_BGR5552BGR as _, "BGR5552BGR");
                    ui.selectable_value(&mut self.code, COLOR_BGR5552RGB as _, "BGR5552RGB");
                    ui.selectable_value(&mut self.code, COLOR_BGRA2BGR555 as _, "BGRA2BGR555");
                    ui.selectable_value(&mut self.code, COLOR_RGBA2BGR555 as _, "RGBA2BGR555");
                    ui.selectable_value(&mut self.code, COLOR_BGR5552BGRA as _, "BGR5552BGRA");
                    ui.selectable_value(&mut self.code, COLOR_BGR5552RGBA as _, "BGR5552RGBA");
                    ui.selectable_value(&mut self.code, COLOR_GRAY2BGR555 as _, "GRAY2BGR555");
                    ui.selectable_value(&mut self.code, COLOR_BGR5552GRAY as _, "BGR5552GRAY");
                    ui.selectable_value(&mut self.code, COLOR_BGR2XYZ as _, "BGR2XYZ");
                    ui.selectable_value(&mut self.code, COLOR_RGB2XYZ as _, "RGB2XYZ");
                    ui.selectable_value(&mut self.code, COLOR_XYZ2BGR as _, "XYZ2BGR");
                    ui.selectable_value(&mut self.code, COLOR_XYZ2RGB as _, "XYZ2RGB");
                    ui.selectable_value(&mut self.code, COLOR_BGR2YCrCb as _, "BGR2YCrCb");
                    ui.selectable_value(&mut self.code, COLOR_RGB2YCrCb as _, "RGB2YCrCb");
                    ui.selectable_value(&mut self.code, COLOR_YCrCb2BGR as _, "YCrCb2BGR");
                    ui.selectable_value(&mut self.code, COLOR_YCrCb2RGB as _, "YCrCb2RGB");
                    ui.selectable_value(&mut self.code, COLOR_BGR2HSV as _, "BGR2HSV");
                    ui.selectable_value(&mut self.code, COLOR_RGB2HSV as _, "RGB2HSV");
                    ui.selectable_value(&mut self.code, COLOR_BGR2Lab as _, "BGR2Lab");
                    ui.selectable_value(&mut self.code, COLOR_RGB2Lab as _, "RGB2Lab");
                    ui.selectable_value(&mut self.code, COLOR_BGR2Luv as _, "BGR2Luv");
                    ui.selectable_value(&mut self.code, COLOR_RGB2Luv as _, "RGB2Luv");
                    ui.selectable_value(&mut self.code, COLOR_BGR2HLS as _, "BGR2HLS");
                    ui.selectable_value(&mut self.code, COLOR_RGB2HLS as _, "RGB2HLS");
                    ui.selectable_value(&mut self.code, COLOR_HSV2BGR as _, "HSV2BGR");
                    ui.selectable_value(&mut self.code, COLOR_HSV2RGB as _, "HSV2RGB");
                    ui.selectable_value(&mut self.code, COLOR_Lab2BGR as _, "Lab2BGR");
                    ui.selectable_value(&mut self.code, COLOR_Lab2RGB as _, "Lab2RGB");
                    ui.selectable_value(&mut self.code, COLOR_Luv2BGR as _, "Luv2BGR");
                    ui.selectable_value(&mut self.code, COLOR_Luv2RGB as _, "Luv2RGB");
                    ui.selectable_value(&mut self.code, COLOR_HLS2BGR as _, "HLS2BGR");
                    ui.selectable_value(&mut self.code, COLOR_HLS2RGB as _, "HLS2RGB");
                    ui.selectable_value(&mut self.code, COLOR_BGR2HSV_FULL as _, "BGR2HSV_FULL");
                    ui.selectable_value(&mut self.code, COLOR_RGB2HSV_FULL as _, "RGB2HSV_FULL");
                    ui.selectable_value(&mut self.code, COLOR_BGR2HLS_FULL as _, "BGR2HLS_FULL");
                    ui.selectable_value(&mut self.code, COLOR_RGB2HLS_FULL as _, "RGB2HLS_FULL");
                    ui.selectable_value(&mut self.code, COLOR_HSV2BGR_FULL as _, "HSV2BGR_FULL");
                    ui.selectable_value(&mut self.code, COLOR_HSV2RGB_FULL as _, "HSV2RGB_FULL");
                    ui.selectable_value(&mut self.code, COLOR_HLS2BGR_FULL as _, "HLS2BGR_FULL");
                    ui.selectable_value(&mut self.code, COLOR_HLS2RGB_FULL as _, "HLS2RGB_FULL");
                    ui.selectable_value(&mut self.code, COLOR_LBGR2Lab as _, "LBGR2Lab");
                    ui.selectable_value(&mut self.code, COLOR_LRGB2Lab as _, "LRGB2Lab");
                    ui.selectable_value(&mut self.code, COLOR_LBGR2Luv as _, "LBGR2Luv");
                    ui.selectable_value(&mut self.code, COLOR_LRGB2Luv as _, "LRGB2Luv");
                    ui.selectable_value(&mut self.code, COLOR_Lab2LBGR as _, "Lab2LBGR");
                    ui.selectable_value(&mut self.code, COLOR_Lab2LRGB as _, "Lab2LRGB");
                    ui.selectable_value(&mut self.code, COLOR_Luv2LBGR as _, "Luv2LBGR");
                    ui.selectable_value(&mut self.code, COLOR_Luv2LRGB as _, "Luv2LRGB");
                    ui.selectable_value(&mut self.code, COLOR_BGR2YUV as _, "BGR2YUV");
                    ui.selectable_value(&mut self.code, COLOR_RGB2YUV as _, "RGB2YUV");
                    ui.selectable_value(&mut self.code, COLOR_YUV2BGR as _, "YUV2BGR");
                    ui.selectable_value(&mut self.code, COLOR_YUV2RGB as _, "YUV2RGB");
                    ui.selectable_value(&mut self.code, COLOR_YUV2RGB_NV12 as _, "YUV2RGB_NV12");
                    ui.selectable_value(&mut self.code, COLOR_YUV2BGR_NV12 as _, "YUV2BGR_NV12");
                    ui.selectable_value(&mut self.code, COLOR_YUV2RGB_NV21 as _, "YUV2RGB_NV21");
                    ui.selectable_value(&mut self.code, COLOR_YUV2BGR_NV21 as _, "YUV2BGR_NV21");
                    ui.selectable_value(&mut self.code, COLOR_YUV2RGBA_NV12 as _, "YUV2RGBA_NV12");
                    ui.selectable_value(&mut self.code, COLOR_YUV2BGRA_NV12 as _, "YUV2BGRA_NV12");
                    ui.selectable_value(&mut self.code, COLOR_YUV2RGBA_NV21 as _, "YUV2RGBA_NV21");
                    ui.selectable_value(&mut self.code, COLOR_YUV2BGRA_NV21 as _, "YUV2BGRA_NV21");
                    ui.selectable_value(&mut self.code, COLOR_YUV2RGB_YV12 as _, "YUV2RGB_YV12");
                    ui.selectable_value(&mut self.code, COLOR_YUV2BGR_YV12 as _, "YUV2BGR_YV12");
                    ui.selectable_value(&mut self.code, COLOR_YUV2RGB_IYUV as _, "YUV2RGB_IYUV");
                    ui.selectable_value(&mut self.code, COLOR_YUV2BGR_IYUV as _, "YUV2BGR_IYUV");
                    ui.selectable_value(&mut self.code, COLOR_YUV2RGBA_YV12 as _, "YUV2RGBA_YV12");
                    ui.selectable_value(&mut self.code, COLOR_YUV2BGRA_YV12 as _, "YUV2BGRA_YV12");
                    ui.selectable_value(&mut self.code, COLOR_YUV2RGBA_IYUV as _, "YUV2RGBA_IYUV");
                    ui.selectable_value(&mut self.code, COLOR_YUV2BGRA_IYUV as _, "YUV2BGRA_IYUV");
                    ui.selectable_value(&mut self.code, COLOR_YUV2GRAY_420 as _, "YUV2GRAY_420");
                    ui.selectable_value(&mut self.code, COLOR_YUV2RGB_UYVY as _, "YUV2RGB_UYVY");
                    ui.selectable_value(&mut self.code, COLOR_YUV2BGR_UYVY as _, "YUV2BGR_UYVY");
                    ui.selectable_value(&mut self.code, COLOR_YUV2RGBA_UYVY as _, "YUV2RGBA_UYVY");
                    ui.selectable_value(&mut self.code, COLOR_YUV2BGRA_UYVY as _, "YUV2BGRA_UYVY");
                    ui.selectable_value(&mut self.code, COLOR_YUV2RGB_YUY2 as _, "YUV2RGB_YUY2");
                    ui.selectable_value(&mut self.code, COLOR_YUV2BGR_YUY2 as _, "YUV2BGR_YUY2");
                    ui.selectable_value(&mut self.code, COLOR_YUV2RGB_YVYU as _, "YUV2RGB_YVYU");
                    ui.selectable_value(&mut self.code, COLOR_YUV2BGR_YVYU as _, "YUV2BGR_YVYU");
                    ui.selectable_value(&mut self.code, COLOR_YUV2RGBA_YUY2 as _, "YUV2RGBA_YUY2");
                    ui.selectable_value(&mut self.code, COLOR_YUV2BGRA_YUY2 as _, "YUV2BGRA_YUY2");
                    ui.selectable_value(&mut self.code, COLOR_YUV2RGBA_YVYU as _, "YUV2RGBA_YVYU");
                    ui.selectable_value(&mut self.code, COLOR_YUV2BGRA_YVYU as _, "YUV2BGRA_YVYU");
                    ui.selectable_value(&mut self.code, COLOR_YUV2GRAY_UYVY as _, "YUV2GRAY_UYVY");
                    ui.selectable_value(&mut self.code, COLOR_YUV2GRAY_YUY2 as _, "YUV2GRAY_YUY2");
                    ui.selectable_value(&mut self.code, COLOR_RGBA2mRGBA as _, "RGBA2mRGBA");
                    ui.selectable_value(&mut self.code, COLOR_mRGBA2RGBA as _, "mRGBA2RGBA");
                    ui.selectable_value(&mut self.code, COLOR_RGB2YUV_I420 as _, "RGB2YUV_I420");
                    ui.selectable_value(&mut self.code, COLOR_BGR2YUV_I420 as _, "BGR2YUV_I420");
                    ui.selectable_value(&mut self.code, COLOR_RGBA2YUV_I420 as _, "RGBA2YUV_I420");
                    ui.selectable_value(&mut self.code, COLOR_BGRA2YUV_I420 as _, "BGRA2YUV_I420");
                    ui.selectable_value(&mut self.code, COLOR_RGB2YUV_YV12 as _, "RGB2YUV_YV12");
                    ui.selectable_value(&mut self.code, COLOR_BGR2YUV_YV12 as _, "BGR2YUV_YV12");
                    ui.selectable_value(&mut self.code, COLOR_RGBA2YUV_YV12 as _, "RGBA2YUV_YV12");
                    ui.selectable_value(&mut self.code, COLOR_BGRA2YUV_YV12 as _, "BGRA2YUV_YV12");
                    ui.selectable_value(&mut self.code, COLOR_BayerBG2BGR as _, "BayerBG2BGR");
                    ui.selectable_value(&mut self.code, COLOR_BayerGB2BGR as _, "BayerGB2BGR");
                    ui.selectable_value(&mut self.code, COLOR_BayerRG2BGR as _, "BayerRG2BGR");
                    ui.selectable_value(&mut self.code, COLOR_BayerGR2BGR as _, "BayerGR2BGR");
                    ui.selectable_value(&mut self.code, COLOR_BayerBG2GRAY as _, "BayerBG2GRAY");
                    ui.selectable_value(&mut self.code, COLOR_BayerGB2GRAY as _, "BayerGB2GRAY");
                    ui.selectable_value(&mut self.code, COLOR_BayerRG2GRAY as _, "BayerRG2GRAY");
                    ui.selectable_value(&mut self.code, COLOR_BayerGR2GRAY as _, "BayerGR2GRAY");
                    ui.selectable_value(&mut self.code, COLOR_BayerBG2BGR_VNG as _, "BayerBG2BGR_VNG");
                    ui.selectable_value(&mut self.code, COLOR_BayerGB2BGR_VNG as _, "BayerGB2BGR_VNG");
                    ui.selectable_value(&mut self.code, COLOR_BayerRG2BGR_VNG as _, "BayerRG2BGR_VNG");
                    ui.selectable_value(&mut self.code, COLOR_BayerGR2BGR_VNG as _, "BayerGR2BGR_VNG");
                    ui.selectable_value(&mut self.code, COLOR_BayerBG2BGR_EA as _, "BayerBG2BGR_EA");
                    ui.selectable_value(&mut self.code, COLOR_BayerGB2BGR_EA as _, "BayerGB2BGR_EA");
                    ui.selectable_value(&mut self.code, COLOR_BayerRG2BGR_EA as _, "BayerRG2BGR_EA");
                    ui.selectable_value(&mut self.code, COLOR_BayerGR2BGR_EA as _, "BayerGR2BGR_EA");
                    ui.selectable_value(&mut self.code, COLOR_BayerBG2BGRA as _, "BayerBG2BGRA");
                    ui.selectable_value(&mut self.code, COLOR_BayerGB2BGRA as _, "BayerGB2BGRA");
                    ui.selectable_value(&mut self.code, COLOR_BayerRG2BGRA as _, "BayerRG2BGRA");
                    ui.selectable_value(&mut self.code, COLOR_BayerGR2BGRA as _, "BayerGR2BGRA");
                    // ui.selectable_value(&mut self.code, COLOR_RGB2YUV_UYVY as _, "RGB2YUV_UYVY");
                    // ui.selectable_value(&mut self.code, COLOR_BGR2YUV_UYVY as _, "BGR2YUV_UYVY");
                    // ui.selectable_value(&mut self.code, COLOR_RGBA2YUV_UYVY as _, "RGBA2YUV_UYVY");
                    // ui.selectable_value(&mut self.code, COLOR_BGRA2YUV_UYVY as _, "BGRA2YUV_UYVY");
                    // ui.selectable_value(&mut self.code, COLOR_RGB2YUV_YUY2 as _, "RGB2YUV_YUY2");
                    // ui.selectable_value(&mut self.code, COLOR_BGR2YUV_YUY2 as _, "BGR2YUV_YUY2");
                    // ui.selectable_value(&mut self.code, COLOR_RGB2YUV_YVYU as _, "RGB2YUV_YVYU");
                    // ui.selectable_value(&mut self.code, COLOR_BGR2YUV_YVYU as _, "BGR2YUV_YVYU");
                    // ui.selectable_value(&mut self.code, COLOR_RGBA2YUV_YUY2 as _, "RGBA2YUV_YUY2");
                    // ui.selectable_value(&mut self.code, COLOR_BGRA2YUV_YUY2 as _, "BGRA2YUV_YUY2");
                    // ui.selectable_value(&mut self.code, COLOR_RGBA2YUV_YVYU as _, "RGBA2YUV_YVYU");
                    // ui.selectable_value(&mut self.code, COLOR_BGRA2YUV_YVYU as _, "BGRA2YUV_YVYU");
                })
                .response
                .on_hover_text("Color conversion code");
        });
    }
}
