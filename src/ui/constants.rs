pub const WINDOW_WIDTH: f32 = 260.0;
// pub const WINDOW_HEIGHT: f32 = 240.0;

pub const BTN_MAIN_HEIGHT: f32 = 36.0;
pub const BTN_HEIGHT: f32 = 26.0;

pub const PADDING_TOP: f32 = 12.0;
pub const PADDING_RATIO: f32 = 0.85;

pub const LABEL_SPACING: f32 = 6.0;
pub const SECTION_SPACING: f32 = 8.0;

#[cfg(target_os = "windows")]
pub const LABEL_APP_LAUNCHER: &str = "Add to Start Menu";
#[cfg(target_os = "linux")]
pub const LABEL_APP_LAUNCHER: &str = "Add to app launcher";
