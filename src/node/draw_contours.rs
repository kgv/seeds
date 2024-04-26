use super::Point;
use crate::{
    utils::SyncMat,
    view::{View, RED, UNTYPED_COLOR},
};
use egui::{DragValue, Ui};
use egui_snarl::{ui::PinInfo, InPin};
use opencv::core::Mat;
use serde::{Deserialize, Serialize};
use std::{
    hash::{Hash, Hasher},
    sync::Arc,
};

/// Draw contours
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DrawContours {
    #[serde(skip)]
    pub contours: Vec<Mat>,
    pub color: [f64; 4],
}

// impl View for DrawContours {
//     fn show_input(&mut self, ui: &mut Ui, pin: &InPin) -> PinInfo {
//         match pin.id.input {
//             0 if pin.remotes.is_empty() => PinInfo::square().with_fill(UNTYPED_COLOR),
//             0 => {
//                 ui.label(self.contours.to_string());
//                 PinInfo::square().with_fill(RED)
//             }
//             _ => unreachable!("DrawContours node has 1 input"),
//         }
//     }

//     fn show_body(&mut self, ui: &mut Ui) {
//         ui.vertical(|ui| {
//             // Kernel
//             ui.horizontal(|ui| {
//                 ui.label("Kernel:");
//                 ui.add(
//                     DragValue::new(&mut self.kernel.rows)
//                         .speed(1)
//                         .clamp_range(0..=i32::MAX),
//                 )
//                 .on_hover_text("Kernel rows");
//                 ui.add(
//                     DragValue::new(&mut self.kernel.cols)
//                         .speed(1)
//                         .clamp_range(0..=i32::MAX),
//                 )
//                 .on_hover_text("Kernel columns");
//             });
//             // Anchor
//             ui.horizontal(|ui| {
//                 ui.label("Anchor:");
//                 ui.add(DragValue::new(&mut self.anchor.x))
//                     .on_hover_text("Anchor x");
//                 ui.add(DragValue::new(&mut self.anchor.y))
//                     .on_hover_text("Anchor y");
//             });
//             // Iterations
//             ui.horizontal(|ui| {
//                 ui.label("Iterations:");
//                 ui.add(
//                     DragValue::new(&mut self.iterations)
//                         .speed(1)
//                         .clamp_range(0..=i32::MAX),
//                 )
//                 .on_hover_text("Iterations");
//             });
//         });
//     }
// }

impl Default for DrawContours {
    fn default() -> Self {
        Self {
            contours: Default::default(),
            color: Default::default(),
        }
    }
}

// impl Hash for DrawContours {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.contours.hash(state);
//         self.color.hash(state);
//     }
// }

// thickness: 1
// line_type: LINE_8
// hierarchy: noArray()
// max_level: INT_MAX
// offset: Point()
