use eframe::NativeOptions;
use eframe::egui::ViewportBuilder;

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
}
