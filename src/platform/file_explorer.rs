use std::path::Path;
use std::process::{Child, Command};

use anyhow::{Context, Result};

#[cfg(target_os = "windows")]
pub fn open_file_explorer(path: &Path) -> Result<Child> {
    Command::new("explorer")
        .arg(path)
        .spawn()
        .context("Failed to open file explorer")
}
#[cfg(target_os = "macos")]
pub fn open_file_explorer(path: &Path) {}
#[cfg(target_os = "linux")]
pub fn open_file_explorer(path: &Path) {}
