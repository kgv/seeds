use super::{View, RED, UNTYPED_COLOR};
use crate::node::ConvertColor;
use egui::Ui;
use egui_snarl::{ui::PinInfo, InPin};

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
}
