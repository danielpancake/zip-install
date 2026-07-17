use eframe::egui;

use crate::app::routing::ViewAction;
use crate::app::state::AppData;

pub mod app_list_view;
pub mod constants;
pub mod dialogs;
pub mod install_view;
pub mod manual_update_view;
pub mod setup_view;
pub mod update_view;
pub mod viewport;

pub trait View {
    fn viewport(&self) -> egui::ViewportBuilder;
    fn ui(&mut self, ui: &mut egui::Ui, data: &mut AppData, action: &mut dyn FnMut(ViewAction));
}
