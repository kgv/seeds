use super::{PinInfoExt, View, RED, UNTYPED_COLOR};
use crate::node::Threshold;
use egui::{DragValue, Ui};
use egui_snarl::{ui::PinInfo, InPin};

impl View for Threshold {
    fn show_input(&mut self, ui: &mut Ui, pin: &InPin) -> PinInfo {
        match pin.id.input {
            0 if pin.remotes.is_empty() => PinInfo::square().with_fill(UNTYPED_COLOR),
            0 => {
                ui.label(self.src.to_string());
                PinInfo::square().with_fill(RED)
            }
            1 => {
                ui.add(
                    DragValue::new(&mut self.thresh)
                        .speed(1.0)
                        .clamp_range(0.0..=self.maxval),
                )
                .on_hover_text("thresh");
                PinInfo::circle().with_fill(UNTYPED_COLOR)
            }
            2 => {
                ui.add(
                    DragValue::new(&mut self.maxval)
                        .speed(1.0)
                        .clamp_range(self.thresh..=255.0),
                )
                .on_hover_text("maxval");
                PinInfo::circle().with_fill(UNTYPED_COLOR)
            }
            _ => unreachable!("Threshold node has 3 inputs"),
        }
    }
}
