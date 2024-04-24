use crate::{
    utils::SyncMat,
    view::{View, RED, UNTYPED_COLOR},
};
use egui::{DragValue, Ui};
use egui_snarl::{ui::PinInfo, InPin};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Median blur
#[derive(Clone, Debug, Deserialize, Hash, Serialize)]
pub struct MedianBlur {
    #[serde(skip)]
    pub src: Arc<SyncMat>,
    pub ksize: i32,
}

impl View for MedianBlur {
    fn show_input(&mut self, ui: &mut Ui, pin: &InPin) -> PinInfo {
        match pin.id.input {
            0 if pin.remotes.is_empty() => PinInfo::square().with_fill(UNTYPED_COLOR),
            0 => {
                ui.label(self.src.to_string());
                PinInfo::square().with_fill(RED)
            }
            _ => unreachable!("MedianBlur node has 1 input"),
        }
    }

    fn show_body(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("K size:");
            ui.add(
                DragValue::new(&mut self.ksize)
                    .speed(2)
                    .clamp_range(3..=999),
            )
            .on_hover_text("ksize");
        });
    }
}

impl Default for MedianBlur {
    fn default() -> Self {
        Self {
            src: Default::default(),
            ksize: 1,
        }
    }
}
