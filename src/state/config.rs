use eframe::egui::ViewportBuilder;
use eframe::NativeOptions;

pub const APP_TITLE: &str = "zip-install";
pub const INSTALL_PATH: &str = "InstalledApps";

pub const MIN_WINDOW_WIDTH: f32 = 240.0;
pub const MIN_WINDOW_HEIGHT: f32 = 300.0;

pub fn default_options() -> NativeOptions {
    NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT])
            .with_resizable(false)
            .with_maximize_button(false)
            .with_minimize_button(false),
        ..Default::default()
    }

    // TODO: spawn window somewhere near the mouse cursor
}
