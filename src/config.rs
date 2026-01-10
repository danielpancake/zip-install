use eframe::NativeOptions;
use eframe::egui::ViewportBuilder;

pub const APP_TITLE: &str = "zip-install";
pub const INSTALL_PATH: &str = "InstalledApps";

pub const WINDOW_WIDTH: f32 = 200.0;
pub const WINDOW_HEIGHT: f32 = 180.0;

pub fn default_options() -> NativeOptions {
    NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_resizable(false)
            .with_maximize_button(false)
            .with_minimize_button(false),
        ..Default::default()
    }

    // TODO: spawn window somewhere near the mouse cursor
}
