#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::*;
#[cfg(target_os = "windows")]
pub use windows::*;

use std::path::PathBuf;

pub struct ContextMenuItem {
    pub label: String,
    pub icon_path: Option<PathBuf>,
    pub executable_path: PathBuf,
}

impl ContextMenuItem {
    pub fn new(label: impl Into<String>, executable_path: impl Into<PathBuf>) -> Self {
        Self {
            label: label.into(),
            icon_path: None,
            executable_path: executable_path.into(),
        }
    }

    pub fn with_icon_path(mut self, icon_path: impl Into<PathBuf>) -> Self {
        self.icon_path = Some(icon_path.into());
        self
    }
}
