use crate::{
    utils::SyncMat,
    view::{View, RED, UNTYPED_COLOR},
};
use egui::Ui;
use egui_snarl::{ui::PinInfo, InPin};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Subtract
#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct Subtract {
    #[serde(skip)]
    pub src1: Arc<SyncMat>,
    #[serde(skip)]
    pub src2: Arc<SyncMat>,
}

impl View for Subtract {
    fn show_input(&mut self, ui: &mut Ui, pin: &InPin) -> PinInfo {
        match pin.id.input {
            0 | 1 if pin.remotes.is_empty() => PinInfo::square().with_fill(UNTYPED_COLOR),
            0 => {
                ui.label(self.src1.to_string());
                PinInfo::square().with_fill(RED)
            }
            1 => {
                ui.label(self.src2.to_string());
                PinInfo::square().with_fill(RED)
            }
            _ => unreachable!("Subtract node has 2 inputs"),
        }
    }
}
