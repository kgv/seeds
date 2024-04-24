use super::Point;
use crate::{
    utils::SyncMat,
    view::{View, RED, UNTYPED_COLOR},
};
use egui::{ComboBox, DragValue, Ui};
use egui_snarl::{ui::PinInfo, InPin};
use opencv::imgproc::{ContourApproximationModes::*, RetrievalModes::*};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Find contours
#[derive(Clone, Debug, Deserialize, Hash, Serialize)]
pub struct FindContours {
    #[serde(skip)]
    pub image: Arc<SyncMat>,
    pub mode: i32,
    pub method: i32,
    pub offset: Point,
}

impl View for FindContours {
    fn show_input(&mut self, ui: &mut Ui, pin: &InPin) -> PinInfo {
        match pin.id.input {
            0 if pin.remotes.is_empty() => PinInfo::square().with_fill(UNTYPED_COLOR),
            0 => {
                ui.label(self.image.to_string());
                PinInfo::square().with_fill(RED)
            }
            _ => unreachable!("FindContours node has 1 input"),
        }
    }

    fn show_body(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            // Mode
            ui.horizontal(|ui| {
                ui.label("Mode:");
                ComboBox::from_id_source("mode")
                    .selected_text(self.mode.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.mode, RETR_EXTERNAL as _, "EXTERNAL");
                        ui.selectable_value(&mut self.mode, RETR_LIST as _, "LIST");
                        ui.selectable_value(&mut self.mode, RETR_CCOMP as _, "CCOMP");
                        ui.selectable_value(&mut self.mode, RETR_TREE as _, "TREE");
                        ui.selectable_value(&mut self.mode, RETR_FLOODFILL as _, "FLOODFILL");
                    })
                    .response
                    .on_hover_text("Mode");
            });
            // Method
            ui.horizontal(|ui| {
                ui.label("Method:");
                ComboBox::from_id_source("method")
                    .selected_text(self.method.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.method, CHAIN_APPROX_NONE as _, "NONE");
                        ui.selectable_value(&mut self.method, CHAIN_APPROX_SIMPLE as _, "SIMPLE");
                        ui.selectable_value(&mut self.method, CHAIN_APPROX_TC89_L1 as _, "TC89_L1");
                        ui.selectable_value(
                            &mut self.method,
                            CHAIN_APPROX_TC89_KCOS as _,
                            "TC89_KCOS",
                        );
                    })
                    .response
                    .on_hover_text("Method");
            });
            // Offset
            ui.horizontal(|ui| {
                ui.label("Offset:");
                ui.add(DragValue::new(&mut self.offset.x))
                    .on_hover_text("Offset x");
                ui.add(DragValue::new(&mut self.offset.y))
                    .on_hover_text("Offset y");
            });
        });
    }
}

impl Default for FindContours {
    fn default() -> Self {
        Self {
            image: Default::default(),
            mode: Default::default(),
            method: CHAIN_APPROX_NONE as _,
            offset: Default::default(),
        }
    }
}
