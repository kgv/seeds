use crate::{
    cache::{
        ConvertColorCache, DilateCache, FindContoursCache, GreaterThanCache, MedianBlurCache,
        ReadCache, SubtractCache, ThresholdCache,
    },
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
use std::{collections::HashSet, path::PathBuf};
use tracing::error;

pub(crate) const RED: Color32 = Color32::from_rgb(0xb0, 0x00, 0x00);
pub(crate) const _COLOR: Color32 = Color32::from_rgb(0xb0, 0xb0, 0x00);
pub(crate) const GREEN: Color32 = Color32::from_rgb(0x00, 0xb0, 0x00);
pub(crate) const IMAGE_COLOR: Color32 = Color32::from_rgb(0xb0, 0x00, 0xb0);
pub(crate) const _BLUE: Color32 = Color32::from_rgb(0x00, 0x00, 0xb0);
pub(crate) const UNTYPED_COLOR: Color32 = Color32::from_rgb(0xb0, 0xb0, 0xb0);

/// View
pub(crate) trait View {
    fn show_input(&mut self, ui: &mut Ui, pin: &InPin) -> PinInfo;

    fn show_body(&mut self, _ui: &mut Ui) {}
}

/// Viewer
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
            Node::Write(_) => 1,
            Node::ConvertColor(_) => 1,
            Node::Dilate(_) => 1,
            Node::FindContours(_) => 1,
            Node::GreaterThan(_) => 1,
            Node::MedianBlur(_) => 1,
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
            ui.memory_mut(|memory| match &snarl[remote.node] {
                Node::Read(read) => match memory.caches.cache::<ReadCache>().get(read) {
                    Ok(out) => *snarl[pin.id.node].as_mat_mut(pin.id.input) = out,
                    Err(error) => error!(%error),
                },
                Node::Write(_write) => unreachable!(),
                Node::ConvertColor(convert_color) => match memory
                    .caches
                    .cache::<ConvertColorCache>()
                    .get(convert_color)
                {
                    Ok(out) => *snarl[pin.id.node].as_mat_mut(pin.id.input) = out,
                    Err(error) => error!(%error),
                },
                Node::Dilate(dilate) => match memory.caches.cache::<DilateCache>().get(dilate) {
                    Ok(out) => *snarl[pin.id.node].as_mat_mut(pin.id.input) = out,
                    Err(error) => error!(%error),
                },
                Node::FindContours(find_contours) => match memory
                    .caches
                    .cache::<FindContoursCache>()
                    .get(find_contours)
                {
                    Ok(_) => {}
                    Err(error) => error!(%error),
                },
                Node::GreaterThan(greater_than) => {
                    match memory.caches.cache::<GreaterThanCache>().get(greater_than) {
                        Ok(out) => *snarl[pin.id.node].as_mat_mut(pin.id.input) = out,
                        Err(error) => error!(%error),
                    }
                }
                Node::MedianBlur(median_blur) => {
                    match memory.caches.cache::<MedianBlurCache>().get(median_blur) {
                        Ok(out) => *snarl[pin.id.node].as_mat_mut(pin.id.input) = out,
                        Err(error) => error!(%error),
                    }
                }
                Node::Subtract(subtract) => {
                    match memory.caches.cache::<SubtractCache>().get(subtract) {
                        Ok(out) => *snarl[pin.id.node].as_mat_mut(pin.id.input) = out,
                        Err(error) => error!(%error),
                    }
                }
                Node::Threshold(threshold) => {
                    match memory.caches.cache::<ThresholdCache>().get(threshold) {
                        Ok(out) => *snarl[pin.id.node].as_mat_mut(pin.id.input) = out,
                        Err(error) => error!(%error),
                    }
                }
            });

            // if let Some(value) =
            //     ui.memory_mut(|memory| memory.caches.cache::<NodeCache>().get(&snarl[remote.node]))
            // {
            //     *snarl[pin.id.node].r#in(pin.id.input) = value;
            // }
        };
        match &mut snarl[pin.id.node] {
            Node::Read(_) => unreachable!("Read node has 0 inputs"),
            Node::Write(write) => write.show_input(ui, pin),
            Node::ConvertColor(convert_color) => convert_color.show_input(ui, pin),
            Node::Dilate(dilate) => dilate.show_input(ui, pin),
            Node::FindContours(find_contours) => find_contours.show_input(ui, pin),
            Node::GreaterThan(greater_than) => greater_than.show_input(ui, pin),
            Node::MedianBlur(median_blur) => median_blur.show_input(ui, pin),
            Node::Subtract(subtract) => subtract.show_input(ui, pin),
            Node::Threshold(threshold) => threshold.show_input(ui, pin),
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
                ui.horizontal(|ui| {
                    let mut text = read.path.to_string_lossy();
                    if ui.text_edit_singleline(&mut text).changed() {
                        read.path = PathBuf::from(&*text);
                    }
                    ui.label("Path:");
                });
                if pin.remotes.is_empty() {
                    return PinInfo::square().with_fill(UNTYPED_COLOR);
                }
                PinInfo::square().with_fill(RED)
            }
            Node::Write(_) => unreachable!("Write node has no outputs"),
            Node::ConvertColor(_) => {
                assert_eq!(pin.id.output, 0, "ConvertColor node has only one output");
                if pin.remotes.is_empty() {
                    return PinInfo::square().with_fill(UNTYPED_COLOR);
                }
                PinInfo::square().with_fill(RED)
            }
            Node::Dilate(_) => {
                assert_eq!(pin.id.output, 0, "ConvertColor node has only one output");
                if pin.remotes.is_empty() {
                    return PinInfo::square().with_fill(UNTYPED_COLOR);
                }
                PinInfo::square().with_fill(RED)
            }
            Node::FindContours(_) => {
                assert_eq!(pin.id.output, 0, "FindContours node has only one output");
                if pin.remotes.is_empty() {
                    return PinInfo::triangle().with_fill(UNTYPED_COLOR);
                }
                PinInfo::triangle().with_fill(GREEN)
            }
            Node::GreaterThan(_) => {
                assert_eq!(pin.id.output, 0, "GreaterThan node has only one output");
                if pin.remotes.is_empty() {
                    return PinInfo::square().with_fill(UNTYPED_COLOR);
                }
                PinInfo::square().with_fill(RED)
            }
            Node::MedianBlur(_) => {
                assert_eq!(pin.id.output, 0, "MedianBlur node has only one output");
                if pin.remotes.is_empty() {
                    return PinInfo::square().with_fill(UNTYPED_COLOR);
                }
                PinInfo::square().with_fill(RED)
            }
            Node::Subtract(_) => {
                assert_eq!(pin.id.output, 0, "Subtract node has only one output");
                if pin.remotes.is_empty() {
                    return PinInfo::square().with_fill(UNTYPED_COLOR);
                }
                PinInfo::square().with_fill(RED)
            }
            Node::Threshold(_) => {
                assert_eq!(pin.id.output, 0, "Threshold node has only one output");
                if pin.remotes.is_empty() {
                    return PinInfo::square().with_fill(UNTYPED_COLOR);
                }
                PinInfo::square().with_fill(RED)
            }
        }
    }

    fn has_footer(&mut self, node: &Node) -> bool {
        match node {
            Node::Write(_) => true,
            Node::ConvertColor(_) => true,
            Node::Dilate(_) => true,
            Node::FindContours(_) => true,
            Node::GreaterThan(_) => true,
            Node::MedianBlur(_) => true,
            Node::Subtract(_) => true,
            Node::Threshold(_) => true,
            _ => false,
        }
    }

    fn show_footer(
        &mut self,
        node: NodeId,
        inputs: &[InPin],
        outputs: &[OutPin],
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<Node>,
    ) {
        match &mut snarl[node] {
            Node::Read(_) => unreachable!("Read node has 0 inputs"),
            Node::Write(write) => write.show_body(ui),
            Node::ConvertColor(convert_color) => convert_color.show_body(ui),
            Node::Dilate(dilate) => dilate.show_body(ui),
            Node::FindContours(find_contours) => find_contours.show_body(ui),
            Node::GreaterThan(greater_than) => greater_than.show_body(ui),
            Node::MedianBlur(median_blur) => median_blur.show_body(ui),
            Node::Subtract(subtract) => subtract.show_body(ui),
            Node::Threshold(threshold) => threshold.show_body(ui),
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
