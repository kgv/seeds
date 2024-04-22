use super::{PinInfoExt, View, MATRIX_COLOR, UNTYPED_COLOR};
use crate::node::FindContours;
use egui::{ComboBox, DragValue, Ui};
use egui_snarl::{ui::PinInfo, InPin};
use opencv::imgproc::{ContourApproximationModes::*, RetrievalModes::*};

impl View for FindContours {
    fn show_input(&mut self, ui: &mut Ui, pin: &InPin) -> PinInfo {
        match pin.id.input {
            0 if pin.remotes.is_empty() => PinInfo::square().with_fill(UNTYPED_COLOR),
            0 => {
                ui.label(self.image.to_string());
                PinInfo::square().with_fill(MATRIX_COLOR)
            }
            1 => {
                ComboBox::from_label("mode")
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
                PinInfo::none()
            }
            2 => {
                ComboBox::from_label("method")
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
                PinInfo::none()
            }
            3 => {
                ui.add(DragValue::new(&mut self.offset.x))
                    .on_hover_text("Offset x");
                ui.add(DragValue::new(&mut self.offset.y))
                    .on_hover_text("Offset y");
                PinInfo::none()
            }
            _ => unreachable!("FindContours node has 4 inputs"),
        }
    }
}
