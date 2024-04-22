use super::{PinInfoExt, View, MATRIX_COLOR, UNTYPED_COLOR};
use crate::node::Dilate;
use egui::{DragValue, Ui};
use egui_snarl::{ui::PinInfo, InPin};

impl View for Dilate {
    fn show_input(&mut self, ui: &mut Ui, pin: &InPin) -> PinInfo {
        match pin.id.input {
            0 if pin.remotes.is_empty() => PinInfo::square().with_fill(UNTYPED_COLOR),
            0 => {
                ui.label(self.src.to_string());
                PinInfo::square().with_fill(MATRIX_COLOR)
            }
            1 => {
                ui.add(
                    DragValue::new(&mut self.kernel.rows)
                        .speed(1)
                        .clamp_range(0..=i32::MAX),
                )
                .on_hover_text("Kernel rows");
                ui.add(
                    DragValue::new(&mut self.kernel.cols)
                        .speed(1)
                        .clamp_range(0..=i32::MAX),
                )
                .on_hover_text("Kernel columns");
                PinInfo::none()
            }
            2 => {
                ui.add(DragValue::new(&mut self.anchor.x))
                    .on_hover_text("Anchor x");
                ui.add(DragValue::new(&mut self.anchor.y))
                    .on_hover_text("Anchor y");
                PinInfo::none()
            }
            3 => {
                ui.add(
                    DragValue::new(&mut self.iterations)
                        .speed(1)
                        .clamp_range(0..=i32::MAX),
                )
                .on_hover_text("Iterations");
                PinInfo::none()
            }
            _ => unreachable!("Dilate node has 4 inputs"),
        }
    }
}
