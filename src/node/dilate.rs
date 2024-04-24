use super::Point;
use crate::{
    utils::SyncMat,
    view::{View, RED, UNTYPED_COLOR},
};
use egui::{DragValue, Ui};
use egui_snarl::{ui::PinInfo, InPin};
use serde::{Deserialize, Serialize};
use std::{
    hash::{Hash, Hasher},
    sync::Arc,
};

/// Dilate
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Dilate {
    #[serde(skip)]
    pub src: Arc<SyncMat>,
    pub kernel: Kernel,
    pub anchor: Point,
    pub iterations: i32,
}

impl View for Dilate {
    fn show_input(&mut self, ui: &mut Ui, pin: &InPin) -> PinInfo {
        match pin.id.input {
            0 if pin.remotes.is_empty() => PinInfo::square().with_fill(UNTYPED_COLOR),
            0 => {
                ui.label(self.src.to_string());
                PinInfo::square().with_fill(RED)
            }
            _ => unreachable!("Dilate node has 1 input"),
        }
    }

    fn show_body(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            // Kernel
            ui.horizontal(|ui| {
                ui.label("Kernel:");
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
            });
            // Anchor
            ui.horizontal(|ui| {
                ui.label("Anchor:");
                ui.add(DragValue::new(&mut self.anchor.x))
                    .on_hover_text("Anchor x");
                ui.add(DragValue::new(&mut self.anchor.y))
                    .on_hover_text("Anchor y");
            });
            // Iterations
            ui.horizontal(|ui| {
                ui.label("Iterations:");
                ui.add(
                    DragValue::new(&mut self.iterations)
                        .speed(1)
                        .clamp_range(0..=i32::MAX),
                )
                .on_hover_text("Iterations");
            });
        });
    }
}

impl Default for Dilate {
    fn default() -> Self {
        Self {
            src: Default::default(),
            kernel: Default::default(),
            anchor: Point { x: -1, y: -1 },
            iterations: Default::default(),
        }
    }
}

impl Hash for Dilate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.src.hash(state);
        self.kernel.hash(state);
        self.anchor.hash(state);
        self.iterations.hash(state);
    }
}

/// Kernel
#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct Kernel {
    pub rows: i32,
    pub cols: i32,
    pub typ: i32,
}
