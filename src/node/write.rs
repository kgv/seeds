use crate::cache::WriteCache;
use crate::{
    utils::SyncMat,
    view::{View, RED, UNTYPED_COLOR},
};
use egui::{Id, Ui};
use egui_snarl::{ui::PinInfo, InPin};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tracing::error;

/// Write
#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct Write {
    #[serde(skip)]
    pub img: Arc<SyncMat>,
    pub path: PathBuf,
}

impl View for Write {
    fn show_input(&mut self, ui: &mut Ui, pin: &InPin) -> PinInfo {
        match pin.id.input {
            0 if pin.remotes.is_empty() => PinInfo::square().with_fill(UNTYPED_COLOR),
            0 => {
                ui.label(self.img.to_string());
                PinInfo::square().with_fill(RED)
            }
            _ => unreachable!("Write node has 1 input"),
        }
    }

    fn show_body(&mut self, ui: &mut Ui) {
        // Path
        ui.horizontal(|ui| {
            ui.label("Path:");
            let mut text = self.path.to_string_lossy();
            if ui.text_edit_singleline(&mut text).changed() {
                self.path = PathBuf::from(&*text)
            }
        });
        // Save
        ui.horizontal(|ui| {
            let clicked = ui.button("Save").clicked();
            let id = Id::new("auto_save");
            let mut checked = ui
                .data_mut(|data| data.get_persisted(id))
                .unwrap_or_default();
            if ui.checkbox(&mut checked, "Auto save").changed() {
                ui.data_mut(|data| data.insert_persisted(id, checked))
            }
            if clicked || checked {
                if let Err(error) =
                    ui.memory_mut(|memory| memory.caches.cache::<WriteCache>().get(self))
                {
                    error!(%error);
                }
            }
        });
    }
}
