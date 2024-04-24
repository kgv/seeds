use crate::{node::Node, view::Viewer};
use clap::crate_version;
use eframe::{get_value, set_value, CreationContext, Frame, Storage, APP_KEY};
use egui::{github_link_file, warn_if_debug_build, Align, CentralPanel, Context, Id, Layout};
use egui_snarl::{ui::SnarlStyle, NodeId, Snarl};
use std::{collections::HashSet, path::PathBuf};

// crate_version!()
pub struct App {
    #[cfg(not(target_arch = "wasm32"))]
    path: Option<PathBuf>,
    snarl: Snarl<Node>,
    removed_node_indices: HashSet<NodeId>,
    updated_node_indices: HashSet<NodeId>,
    version: usize,
}

impl App {
    pub fn new(#[allow(unused_variables)] cc: &CreationContext) -> Self {
        let snarl: Snarl<Node> = cc
            .storage
            .and_then(|storage| get_value(storage, APP_KEY))
            .unwrap_or_default();
        let removed_node_indices = Default::default();
        let updated_node_indices = Default::default();
        // let updated_node_indices = snarl
        //     .node_ids()
        //     .filter_map(|(node_idx, node)| node.has_image().then_some(node_idx))
        //     .collect();
        let snarl = Default::default();
        Self {
            #[cfg(not(target_arch = "wasm32"))]
            path: None,
            snarl,
            removed_node_indices,
            updated_node_indices,
            version: 0,
        }
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn Storage) {
        set_value(storage, APP_KEY, &self.snarl);
    }

    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.snarl.show(
                &mut Viewer {
                    removed_ids: &mut self.removed_node_indices,
                    updated_ids: &mut self.updated_node_indices,
                },
                &SnarlStyle {
                    _collapsible: Some(true),
                    ..Default::default()
                },
                Id::new("snarl"),
                ui,
            );
            ui.with_layout(Layout::bottom_up(Align::RIGHT), |ui| {
                warn_if_debug_build(ui);
            });
        });
        // if self.has_changes() {
        //     self.remove_nodes();
        //     self.update_nodes(ctx);
        // }
    }
}
