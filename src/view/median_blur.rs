use super::{PinInfoExt, View, MATRIX_COLOR, UNTYPED_COLOR};
use crate::node::MedianBlur;
use egui::{DragValue, Ui};
use egui_snarl::{ui::PinInfo, InPin};

impl View for MedianBlur {
    fn show_input(&mut self, ui: &mut Ui, pin: &InPin) -> PinInfo {
        match pin.id.input {
            0 if pin.remotes.is_empty() => PinInfo::square().with_fill(UNTYPED_COLOR),
            0 => {
                ui.label(self.src.to_string());
                PinInfo::square().with_fill(MATRIX_COLOR)
            }
            1 => {
                ui.add(
                    DragValue::new(&mut self.ksize)
                        .speed(2)
                        .clamp_range(3..=999),
                )
                .on_hover_text("ksize");
                PinInfo::none()
            }
            _ => unreachable!("MedianBlur node has 2 inputs"),
        }
    }
}
