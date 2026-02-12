use std::path::Path;

use anyhow::Result;

#[cfg(target_os = "windows")]
pub fn create_shortcut(src: &Path, dest: &Path) -> Result<()> {
    let shell_link = mslnk::ShellLink::new(src)?;
    shell_link.create_lnk(dest.with_extension("lnk"))?;
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn create_shortcut(_src: &str, _dest: &str) -> Result<()> {
    Err(anyhow::anyhow!("Not implemented yet for macOS"))
}

#[cfg(target_os = "linux")]
pub fn create_shortcut(_src: &str, _dest: &str) -> Result<()> {
    Err(anyhow::anyhow!("Not implemented yet for Linux"))
}
