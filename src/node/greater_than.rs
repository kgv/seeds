use crate::{
    utils::SyncMat,
    view::{View, RED, UNTYPED_COLOR},
};
use egui::{epaint::util::FloatOrd, DragValue, Ui};
use egui_snarl::{ui::PinInfo, InPin};
use serde::{Deserialize, Serialize};
use std::{
    hash::{Hash, Hasher},
    sync::Arc,
};

/// Greater than
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GreaterThan {
    #[serde(skip)]
    pub a: Arc<SyncMat>,
    pub s: f64,
}

impl View for GreaterThan {
    fn show_input(&mut self, ui: &mut Ui, pin: &InPin) -> PinInfo {
        match pin.id.input {
            0 if pin.remotes.is_empty() => PinInfo::square().with_fill(UNTYPED_COLOR),
            0 => {
                ui.label(self.a.to_string());
                PinInfo::square().with_fill(RED)
            }
            _ => unreachable!("GreaterThan node has 1 input"),
        }
    }

    fn show_body(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("S:");
            ui.add(DragValue::new(&mut self.s).speed(2).clamp_range(3..=999))
                .on_hover_text("s");
        });
    }
}

impl Hash for GreaterThan {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.a.hash(state);
        self.s.ord().hash(state);
    }
}
