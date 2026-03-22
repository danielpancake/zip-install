use eframe::egui;

use crate::app::routing::{ViewAction, ViewContext};

pub mod constants;
pub mod dialogs;
pub mod install_view;
pub mod manual_update_view;
pub mod update_view;
pub mod viewport;

pub trait View {
    fn viewport(&self) -> egui::ViewportBuilder;
    fn ui(&mut self, ui: &mut egui::Ui, ctx: &mut ViewContext, action: &mut dyn FnMut(ViewAction));
}
