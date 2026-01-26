use eframe::egui;

use crate::app::routing::ViewAction;

pub mod dialogs;
pub mod self_install_app;
pub mod zip_install;
pub trait View {
    fn viewport(&self) -> egui::ViewportBuilder;
    fn ui(&mut self, ui: &mut egui::Ui, action: &mut dyn FnMut(ViewAction));
}
