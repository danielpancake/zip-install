use anyhow::Result;
use std::path::Path;

#[cfg(target_os = "windows")]
use mslnk::ShellLink;

pub fn create_shortcut(src: &Path, dest: &Path) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        create_windows_shortcut(src, dest)
    }

    #[cfg(target_os = "macos")]
    {
        create_macos_shortcut(src, dest)
    }

    #[cfg(target_os = "linux")]
    {
        create_linux_shortcut(src, dest)
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err(anyhow::anyhow!("Unsupported operating system"))
    }
}

#[cfg(target_os = "windows")]
pub fn create_windows_shortcut(src: &Path, dest: &Path) -> Result<()> {
    let shell_link = ShellLink::new(src)?;
    shell_link.create_lnk(dest)?;
    Ok(())
}

// TODO: check whether it even makes sense there
#[cfg(target_os = "macos")]
pub fn create_macos_shortcut(_src: &str, _dest: &str) -> Result<()> {
    Err(anyhow::anyhow!("Not implemented yet for macOS"))
}

#[cfg(target_os = "linux")]
pub fn create_linux_shortcut(_src: &str, _dest: &str) -> Result<()> {
    Err(anyhow::anyhow!("Not implemented yet for Linux"))
}
