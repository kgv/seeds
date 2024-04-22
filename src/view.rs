use crate::{
    cache::NodeCache,
    node::{
        ConvertColor, Dilate, FindContours, GreaterThan, MedianBlur, Node, Read, Subtract,
        Threshold, Write,
    },
    utils::SyncMat,
};
use egui::{
    util::cache::{CacheTrait, ComputerMut, FrameCache},
    Color32, ComboBox, DragValue, Id, Pos2, Ui,
};
use egui_snarl::{
    ui::{PinInfo, SnarlViewer},
    InPin, NodeId, OutPin, Snarl,
};
use opencv::{
    core::{BorderTypes, Mat, MatExprTraitConst, MatTraitConst},
    imgproc::{
        RetrievalModes, CHAIN_APPROX_NONE, CHAIN_APPROX_SIMPLE, CHAIN_APPROX_TC89_KCOS,
        CHAIN_APPROX_TC89_L1, RETR_CCOMP, RETR_EXTERNAL, RETR_FLOODFILL, RETR_LIST, RETR_TREE,
    },
};
use std::{
    collections::HashSet,
    hash::Hash,
    mem::{replace, take, zeroed, MaybeUninit},
    path::PathBuf,
    sync::Arc,
};
use tracing::error;

const MATRIX_COLOR: Color32 = Color32::from_rgb(0xb0, 0x00, 0x00);
const _COLOR: Color32 = Color32::from_rgb(0xb0, 0xb0, 0x00);
const VECTOR_COLOR: Color32 = Color32::from_rgb(0x00, 0xb0, 0x00);
const IMAGE_COLOR: Color32 = Color32::from_rgb(0xb0, 0x00, 0xb0);
const UNTYPED_COLOR: Color32 = Color32::from_rgb(0xb0, 0xb0, 0xb0);

pub struct Viewer<'a> {
    pub removed_ids: &'a mut HashSet<NodeId>,
    pub updated_ids: &'a mut HashSet<NodeId>,
}

impl<'a> SnarlViewer<Node> for Viewer<'a> {
    fn title(&mut self, node: &Node) -> String {
        match node {
            Node::Read(_) => "Read".to_owned(),
            Node::Write(_) => "Write".to_owned(),
            Node::ConvertColor(_) => "Convert color".to_owned(),
            Node::Dilate(_) => "Dilate".to_owned(),
            Node::FindContours(_) => "Find contours".to_owned(),
            Node::GreaterThan(_) => "Greater than".to_owned(),
            Node::MedianBlur(_) => "Median blur".to_owned(),
            Node::Subtract(_) => "Subtract".to_owned(),
            Node::Threshold(_) => "Threshold".to_owned(),
        }
    }

    #[inline]
    fn connect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<Node>) {
        // Validate connection
        // match (&snarl[from.id.node], &snarl[to.id.node]) {
        //     (Node::ConvertColor, _) => {
        //         unreachable!("Sink node has no outputs")
        //     }
        // }
        for &remote in &to.remotes {
            snarl.disconnect(remote, to.id);
        }
        snarl.connect(from.id, to.id);
    }

    fn disconnect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<Node>) {
        snarl.disconnect(from.id, to.id);
        self.updated_ids.insert(to.id.node);
    }

    fn drop_inputs(&mut self, pin: &InPin, snarl: &mut Snarl<Node>) {
        snarl.drop_inputs(pin.id);
        self.updated_ids.insert(pin.id.node);
    }

    fn drop_outputs(&mut self, pin: &OutPin, snarl: &mut Snarl<Node>) {
        snarl.drop_outputs(pin.id);
        self.updated_ids
            .extend(pin.remotes.iter().map(|remote| remote.node));
    }

    fn inputs(&mut self, node: &Node) -> usize {
        match node {
            Node::Read(_) => 0,
            Node::Write(_) => 3,
            Node::ConvertColor(_) => 1,
            Node::Dilate(_) => 4,
            Node::FindContours(_) => 4,
            Node::GreaterThan(_) => 2,
            Node::MedianBlur(_) => 2,
            Node::Subtract(_) => 2,
            Node::Threshold(_) => 1,
        }
    }

    fn outputs(&mut self, node: &Node) -> usize {
        match node {
            Node::Write(_) => 0,
            _ => 1,
        }
    }

    fn show_input(
        &mut self,
        pin: &InPin,
        ui: &mut Ui,
        _scale: f32,
        snarl: &mut Snarl<Node>,
    ) -> PinInfo {
        if let Some(remote) = pin.remotes.get(0) {
            if let Some(value) =
                ui.memory_mut(|memory| memory.caches.cache::<NodeCache>().get(&snarl[remote.node]))
            {
                *snarl[pin.id.node].r#in(pin.id.input) = value;
            }
        };
        //  else {
        //     *snarl[pin.id.node].r#in(pin.id.input) = Default::default();
        // };
        // if let Node::Write(Write { img, .. }) = &snarl[pin.id.node] {
        //     tracing::error!("Write {img}");
        // }
        match &mut snarl[pin.id.node] {
            Node::Read(_) => unreachable!("Read node has 0 inputs"),
            Node::Write(Write { img, path }) => match pin.id.input {
                0 if pin.remotes.is_empty() => PinInfo::square().with_fill(UNTYPED_COLOR),
                0 => {
                    ui.label(format!("{img}"));
                    PinInfo::square().with_fill(MATRIX_COLOR)
                }
                1 => {
                    let mut text = path.to_string_lossy();
                    if ui.text_edit_singleline(&mut text).changed() {
                        *path = PathBuf::from(&*text)
                    }
                    PinInfo::none()
                }
                2 => {
                    if ui.button("Save").clicked() {
                        ui.memory_mut(|memory| {
                            memory.caches.cache::<NodeCache>().get(&snarl[pin.id.node])
                        });
                    }
                    let id = Id::new("auto_save");
                    let mut checked = ui.data(|data| data.get_temp(id)).unwrap_or_default();
                    if ui.checkbox(&mut checked, "Auto save").changed() {
                        ui.data_mut(|data| data.insert_temp(id, checked))
                    }
                    if checked {
                        ui.memory_mut(|memory| {
                            memory.caches.cache::<NodeCache>().get(&snarl[pin.id.node]);
                        });
                    }
                    PinInfo::none()
                }
                _ => unreachable!("Write node has 3 inputs"),
            },
            Node::ConvertColor(ConvertColor { src, from, to, .. }) => match pin.id.input {
                0 if pin.remotes.is_empty() => PinInfo::square().with_fill(UNTYPED_COLOR),
                0 => {
                    ui.label(format!("{src}"));
                    PinInfo::square().with_fill(MATRIX_COLOR)
                }
                _ => unreachable!("ConvertColor node has only 1 input"),
            },
            Node::Dilate(dilate) => dilate.show_input(ui, pin),
            Node::FindContours(find_contours) => find_contours.show_input(ui, pin),
            Node::GreaterThan(greater_than) => greater_than.show_input(ui, pin),
            Node::MedianBlur(median_blur) => median_blur.show_input(ui, pin),
            Node::Subtract(_) => {
                assert!(pin.id.input < 2, "Subtract node has 2 inputs");
                if pin.remotes.is_empty() {
                    PinInfo::square().with_fill(UNTYPED_COLOR)
                } else {
                    PinInfo::square().with_fill(MATRIX_COLOR)
                }
            }
            Node::Threshold(Threshold { thresh, maxval, .. }) => {
                assert_eq!(pin.id.input, 0, "Threshold node has only 1 input");
                ui.add(DragValue::new(thresh).speed(1.0).clamp_range(0.0..=*maxval))
                    .on_hover_text("thresh");
                ui.add(
                    DragValue::new(maxval)
                        .speed(1.0)
                        .clamp_range(*thresh..=255.0),
                )
                .on_hover_text("maxval");
                if pin.remotes.is_empty() {
                    PinInfo::square().with_fill(UNTYPED_COLOR)
                } else {
                    PinInfo::square().with_fill(MATRIX_COLOR)
                }
            }
        }
    }

    fn show_output(
        &mut self,
        pin: &OutPin,
        ui: &mut Ui,
        _scale: f32,
        snarl: &mut Snarl<Node>,
    ) -> PinInfo {
        match &mut snarl[pin.id.node] {
            Node::Read(read) => {
                assert_eq!(pin.id.output, 0, "Read node has only one output");
                let mut text = read.path.to_string_lossy();
                if ui.text_edit_singleline(&mut text).changed() {
                    read.path = PathBuf::from(&*text);
                }
                if pin.remotes.is_empty() {
                    return PinInfo::square().with_fill(UNTYPED_COLOR);
                }
                PinInfo::square().with_fill(MATRIX_COLOR)
            }
            Node::Write(_) => unreachable!("Write node has no outputs"),
            Node::ConvertColor(_) => {
                assert_eq!(pin.id.output, 0, "ConvertColor node has only one output");
                if pin.remotes.is_empty() {
                    return PinInfo::square().with_fill(UNTYPED_COLOR);
                }
                PinInfo::square().with_fill(MATRIX_COLOR)
            }
            Node::Dilate(_) => {
                assert_eq!(pin.id.output, 0, "ConvertColor node has only one output");
                if pin.remotes.is_empty() {
                    return PinInfo::square().with_fill(UNTYPED_COLOR);
                }
                PinInfo::square().with_fill(MATRIX_COLOR)
            }
            Node::FindContours(_) => {
                assert_eq!(pin.id.output, 0, "FindContours node has only one output");
                if pin.remotes.is_empty() {
                    return PinInfo::triangle().with_fill(UNTYPED_COLOR);
                }
                PinInfo::triangle().with_fill(VECTOR_COLOR)
            }
            Node::GreaterThan(_) => {
                assert_eq!(pin.id.output, 0, "GreaterThan node has only one output");
                if pin.remotes.is_empty() {
                    return PinInfo::square().with_fill(UNTYPED_COLOR);
                }
                PinInfo::square().with_fill(MATRIX_COLOR)
            }
            Node::MedianBlur(_) => {
                assert_eq!(pin.id.output, 0, "MedianBlur node has only one output");
                if pin.remotes.is_empty() {
                    return PinInfo::square().with_fill(UNTYPED_COLOR);
                }
                PinInfo::square().with_fill(MATRIX_COLOR)
            }
            Node::Subtract(_) => {
                assert_eq!(pin.id.output, 0, "Subtract node has only one output");
                if pin.remotes.is_empty() {
                    return PinInfo::square().with_fill(UNTYPED_COLOR);
                }
                PinInfo::square().with_fill(MATRIX_COLOR)
            }
            Node::Threshold(_) => {
                assert_eq!(pin.id.output, 0, "Threshold node has only one output");
                if pin.remotes.is_empty() {
                    return PinInfo::square().with_fill(UNTYPED_COLOR);
                }
                PinInfo::square().with_fill(MATRIX_COLOR)
            }
        }
    }

    fn has_graph_menu(&mut self, _pos: Pos2, _snarl: &mut Snarl<Node>) -> bool {
        true
    }

    fn show_graph_menu(&mut self, pos: Pos2, ui: &mut Ui, _scale: f32, snarl: &mut Snarl<Node>) {
        ui.label("Add node");
        ui.menu_button("Codecs", |ui| {
            if ui.button("Read").clicked() {
                self.updated_ids
                    .insert(snarl.insert_node(pos, Node::Read(Default::default())));
                ui.close_menu();
            }
            if ui.button("Write").clicked() {
                self.updated_ids
                    .insert(snarl.insert_node(pos, Node::Write(Default::default())));
                ui.close_menu();
            }
        });
        ui.menu_button("Proc", |ui| {
            if ui.button("Convert color").clicked() {
                self.updated_ids
                    .insert(snarl.insert_node(pos, Node::ConvertColor(Default::default())));
                ui.close_menu();
            }
            if ui.button("Dilate").clicked() {
                self.updated_ids
                    .insert(snarl.insert_node(pos, Node::Dilate(Default::default())));
                ui.close_menu();
            }
            if ui.button("Find contours").clicked() {
                self.updated_ids
                    .insert(snarl.insert_node(pos, Node::FindContours(Default::default())));
                ui.close_menu();
            }
            if ui.button("Greater than").clicked() {
                self.updated_ids
                    .insert(snarl.insert_node(pos, Node::GreaterThan(Default::default())));
                ui.close_menu();
            }
            if ui.button("Median blur").clicked() {
                self.updated_ids
                    .insert(snarl.insert_node(pos, Node::MedianBlur(Default::default())));
                ui.close_menu();
            }
            if ui.button("Subtract").clicked() {
                self.updated_ids
                    .insert(snarl.insert_node(pos, Node::Subtract(Default::default())));
                ui.close_menu();
            }
            if ui.button("Threshold").clicked() {
                self.updated_ids
                    .insert(snarl.insert_node(pos, Node::Threshold(Default::default())));
                ui.close_menu();
            }
        });
    }

    fn has_node_menu(&mut self, _node: &Node) -> bool {
        true
    }

    fn show_node_menu(
        &mut self,
        node_idx: NodeId,
        inputs: &[InPin],
        outputs: &[OutPin],
        ui: &mut Ui,
        _scale: f32,
        snarl: &mut Snarl<Node>,
    ) {
        ui.label("Node menu");

        let node = snarl.get_node(node_idx);

        if ui.button("Remove").clicked() {
            self.removed_ids.insert(node_idx);
            snarl.remove_node(node_idx);
            ui.close_menu();
        }
    }
}

pub trait View {
    fn show_input(&mut self, ui: &mut Ui, pin: &InPin) -> PinInfo;
}

/// Ext for [`PinInfo`]
pub trait PinInfoExt {
    fn none() -> PinInfo;
}

impl PinInfoExt for PinInfo {
    fn none() -> PinInfo {
        PinInfo::none()
    }
}

mod dilate;
mod find_contours;
mod greater_than;
mod median_blur;

// trait UiExt {
//     fn update<C, K>(&mut self, snarl: &mut Snarl<Node>, pin: &OutPin, key: impl Copy + Hash)
//     where
//         C: ComputerMut<K, Option<Arc<SyncMat>>> + Default;
//     // C: CacheTrait + Default,
//     // FrameCache<Option<Arc<SyncMat>>, T>: ComputerMut<K, Option<Arc<SyncMat>>>;
// }

// impl UiExt for Ui {
//     fn update<C, K>(&mut self, snarl: &mut Snarl<Node>, pin: &OutPin, key: impl Copy + Hash)
//     where
//         C: ComputerMut<K, Option<Arc<SyncMat>>> + Default,
//         // C: CacheTrait + Default,
//         // FrameCache<Option<Arc<SyncMat>>, T>: ComputerMut<K, Option<Arc<SyncMat>>>,
//     {
//         if let Some(out) = self.memory_mut(|memory| {
//             memory
//                 .caches
//                 .cache::<FrameCache<Option<Arc<SyncMat>>, C>>()
//                 .get(key)
//         }) {
//             for remote in &pin.remotes {
//                 *snarl[remote.node].r#in(0) = out.clone();
//             }
//         }
//     }
// }
