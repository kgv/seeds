use crate::{
    cache::NodeCache,
    node::{GreaterThan, MedianBlur, Node, Read, Threshold, Write},
    utils::SyncMat,
};
use egui::{
    util::cache::{CacheTrait, ComputerMut, FrameCache},
    Color32, DragValue, Pos2, Ui,
};
use egui_snarl::{
    ui::{PinInfo, SnarlViewer},
    InPin, NodeId, OutPin, Snarl,
};
use std::{collections::HashSet, hash::Hash, mem::take, path::PathBuf, sync::Arc};
use tracing::error;

const MATRIX_COLOR: Color32 = Color32::from_rgb(0xb0, 0x00, 0x00);
const _1_COLOR: Color32 = Color32::from_rgb(0xb0, 0xb0, 0x00);
const NUMBER_COLOR: Color32 = Color32::from_rgb(0x00, 0xb0, 0x00);
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
            Node::Subtract(_) => 2,
            _ => 1,
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
        match snarl[pin.id.node] {
            Node::Read(_) => unreachable!("Read node has no inputs"),
            Node::Write(write) => {
                assert_eq!(pin.id.input, 0, "Write node has only one input");
                if pin.remotes.is_empty() {
                    return PinInfo::square().with_fill(UNTYPED_COLOR);
                }
                let mut text = write.path.to_string_lossy();
                if ui.text_edit_singleline(&mut text).changed() {
                    write.path = PathBuf::from(&*text);
                }
                if ui.button("Save").clicked() {
                    // ui.memory_mut(|memory| memory.caches.cache::<WriteCache>().update());
                }
                // ui.memory_mut(|memory| memory.caches.cache::<NodeCache>().get(node));
                PinInfo::square().with_fill(MATRIX_COLOR)
            }
            Node::ConvertColor(_) => {
                assert_eq!(pin.id.input, 0, "ConvertColor node has only one input");
                if pin.remotes.is_empty() {
                    return PinInfo::square().with_fill(UNTYPED_COLOR);
                }
                PinInfo::square().with_fill(MATRIX_COLOR)
            }
            Node::GreaterThan(GreaterThan { s, .. }) => {
                assert_eq!(pin.id.input, 0, "GreaterThan node has only one input");
                let value = snarl[pin.id.node].string_in();
                ui.add(DragValue::new(value).speed(2).clamp_range(3..=999))
                    .on_hover_text("s");
                if pin.remotes.is_empty() {
                    return PinInfo::square().with_fill(UNTYPED_COLOR);
                }
                PinInfo::square().with_fill(MATRIX_COLOR)
            }
            Node::MedianBlur(MedianBlur { ksize, .. }) => {
                assert_eq!(pin.id.input, 0, "MedianBlur node has only one input");
                ui.add(DragValue::new(ksize).speed(2).clamp_range(3..=999))
                    .on_hover_text("ksize");
                if pin.remotes.is_empty() {
                    return PinInfo::square().with_fill(UNTYPED_COLOR);
                }
                PinInfo::square().with_fill(MATRIX_COLOR)
            }
            Node::Subtract(subtract) => {
                assert!(
                    matches!(pin.id.input, 0 | 1),
                    "Subtract node has two inputs",
                );
                if pin.remotes.is_empty() {
                    return PinInfo::square().with_fill(UNTYPED_COLOR);
                }
                PinInfo::square().with_fill(MATRIX_COLOR)
            }
            Node::Threshold(Threshold {
                src,
                thresh,
                maxval,
            }) => {
                assert_eq!(pin.id.input, 0, "Threshold node has only one input");
                ui.add(DragValue::new(thresh).speed(1.0).clamp_range(0.0..=*maxval))
                    .on_hover_text("thresh");
                ui.add(
                    DragValue::new(maxval)
                        .speed(1.0)
                        .clamp_range(*thresh..=255.0),
                )
                .on_hover_text("maxval");
                if pin.remotes.is_empty() {
                    return PinInfo::square().with_fill(UNTYPED_COLOR);
                }
                // let src = take(src);
                let key = &snarl[pin.remotes[0].node];
                *src = ui
                    .memory_mut(|memory| memory.caches.cache::<NodeCache>().get(key))
                    .unwrap_or_default();
                PinInfo::square().with_fill(MATRIX_COLOR)
            }
        }
    }

    fn show_output(
        &mut self,
        pin: &OutPin,
        ui: &mut Ui,
        scale: f32,
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
                // if let Some(out) =
                //     ui.memory_mut(|memory| memory.caches.cache::<ReadCache>().get(read))
                // {
                //     for (index, remote) in pin.remotes.iter().enumerate() {
                //         *snarl[remote.node].src(index) = out.clone();
                //     }
                // }
                PinInfo::square().with_fill(MATRIX_COLOR)
            }
            Node::Write(_) => unreachable!("Write node has no outputs"),
            Node::ConvertColor(convert_color) => {
                assert_eq!(pin.id.output, 0, "ConvertColor node has only one output");
                if pin.remotes.is_empty() {
                    return PinInfo::square().with_fill(UNTYPED_COLOR);
                }
                // if let Some(out) = ui.memory_mut(|memory| {
                //     memory
                //         .caches
                //         .cache::<ConvertColorCache>()
                //         .get(convert_color)
                // }) {
                //     for (index, remote) in pin.remotes.iter().enumerate() {
                //         *snarl[remote.node].src(index) = out.clone();
                //     }
                // }
                PinInfo::square().with_fill(MATRIX_COLOR)
            }
            Node::GreaterThan(greater_than) => {
                assert_eq!(pin.id.output, 0, "GreaterThan node has only one output");
                if pin.remotes.is_empty() {
                    return PinInfo::square().with_fill(UNTYPED_COLOR);
                }
                // if let Some(out) = ui.memory_mut(|memory| {
                //     memory.caches.cache::<GreaterThanCache>().get(greater_than)
                // }) {
                //     for (index, remote) in pin.remotes.iter().enumerate() {
                //         *snarl[remote.node].src(index) = out.clone();
                //     }
                // }
                PinInfo::square().with_fill(MATRIX_COLOR)
            }
            Node::MedianBlur(median_blur) => {
                assert_eq!(pin.id.output, 0, "MedianBlur node has only one output");
                if pin.remotes.is_empty() {
                    return PinInfo::square().with_fill(UNTYPED_COLOR);
                }
                // if let Some(out) = ui
                //     .memory_mut(|memory| memory.caches.cache::<MedianBlurCache>().get(median_blur))
                // {
                //     for (index, remote) in pin.remotes.iter().enumerate() {
                //         *snarl[remote.node].src(index) = out.clone();
                //     }
                // }
                PinInfo::square().with_fill(MATRIX_COLOR)
            }
            Node::Subtract(subtract) => {
                assert_eq!(pin.id.output, 0, "Subtract node has only one output");
                if pin.remotes.is_empty() {
                    return PinInfo::square().with_fill(UNTYPED_COLOR);
                }
                // if let Some(out) =
                //     ui.memory_mut(|memory| memory.caches.cache::<SubtractCache>().get(subtract))
                // {
                //     for (index, remote) in pin.remotes.iter().enumerate() {
                //         *snarl[remote.node].src(index) = out.clone();
                //     }
                // }
                PinInfo::square().with_fill(MATRIX_COLOR)
            }
            Node::Threshold(threshold) => {
                assert_eq!(pin.id.output, 0, "Threshold node has only one output");
                if pin.remotes.is_empty() {
                    return PinInfo::square().with_fill(UNTYPED_COLOR);
                }
                // if let Some(out) =
                //     ui.memory_mut(|memory| memory.caches.cache::<ThresholdCache>().get(threshold))
                // {
                //     for (index, remote) in pin.remotes.iter().enumerate() {
                //         *snarl[remote.node].src(index) = out.clone();
                //     }
                // }
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
