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

/// Threshold
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Threshold {
    #[serde(skip)]
    pub src: Arc<SyncMat>,
    pub thresh: f64,
    pub maxval: f64,
}

impl View for Threshold {
    fn show_input(&mut self, ui: &mut Ui, pin: &InPin) -> PinInfo {
        match pin.id.input {
            0 if pin.remotes.is_empty() => PinInfo::square().with_fill(UNTYPED_COLOR),
            0 => {
                ui.label(self.src.to_string());
                PinInfo::square().with_fill(RED)
            }
            _ => unreachable!("Threshold node has 1 input"),
        }
    }

    fn show_body(&mut self, ui: &mut Ui) {
        // Thresh
        ui.add(
            DragValue::new(&mut self.thresh)
                .speed(1.0)
                .clamp_range(0.0..=self.maxval),
        )
        .on_hover_text("thresh");
        // Max value
        ui.add(
            DragValue::new(&mut self.maxval)
                .speed(1.0)
                .clamp_range(self.thresh..=255.0),
        )
        .on_hover_text("maxval");
    }
}

impl Default for Threshold {
    fn default() -> Self {
        Self {
            src: Default::default(),
            thresh: 0.0,
            maxval: 255.0,
        }
    }
}

impl Hash for Threshold {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.src.hash(state);
        self.thresh.ord().hash(state);
        self.maxval.ord().hash(state);
    }
}
