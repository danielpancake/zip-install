use anyhow::{Context, Result};
use std::path::Path;
use std::process::{Child, Command};

#[cfg(target_os = "windows")]
pub fn open_file_explorer(path: &Path) -> Result<Child> {
    Command::new("explorer")
        .arg(path)
        .spawn()
        .context("Failed to open file explorer")
}

#[cfg(target_os = "linux")]
pub fn open_file_explorer(path: &Path) -> Result<Child> {
    Command::new("xdg-open")
        .arg(path)
        .spawn()
        .context("Failed to open file manager")
}
