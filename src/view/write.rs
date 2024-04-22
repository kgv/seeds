use super::{PinInfoExt, View, RED, UNTYPED_COLOR};
use crate::{cache::WriteCache, node::Write};
use egui::{Id, Ui};
use egui_snarl::{ui::PinInfo, InPin};
use std::path::PathBuf;
use tracing::error;

impl View for Write {
    fn show_input(&mut self, ui: &mut Ui, pin: &InPin) -> PinInfo {
        match pin.id.input {
            0 if pin.remotes.is_empty() => PinInfo::square().with_fill(UNTYPED_COLOR),
            0 => {
                ui.label(self.img.to_string());
                PinInfo::square().with_fill(RED)
            }
            1 => {
                let mut text = self.path.to_string_lossy();
                if ui.text_edit_singleline(&mut text).changed() {
                    self.path = PathBuf::from(&*text)
                }
                PinInfo::none()
            }
            2 => {
                let clicked = ui.button("Save").clicked();
                let id = Id::new("auto_save");
                let mut checked = ui.data(|data| data.get_temp(id)).unwrap_or_default();
                if ui.checkbox(&mut checked, "Auto save").changed() {
                    ui.data_mut(|data| data.insert_temp(id, checked))
                }
                if clicked || checked {
                    if let Err(error) =
                        ui.memory_mut(|memory| memory.caches.cache::<WriteCache>().get(self))
                    {
                        error!(%error);
                    }
                }
                PinInfo::none()
            }
            _ => unreachable!("Write node has 3 inputs"),
        }
    }
}
