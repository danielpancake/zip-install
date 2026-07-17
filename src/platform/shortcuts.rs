use std::path::{Path, PathBuf};

use anyhow::Result;

/// Creates a shortcut to `src` at `dest` (extension adjusted per platform)
/// and returns the path of the file actually created.
#[cfg(target_os = "windows")]
pub fn create_shortcut(src: &Path, dest: &Path) -> Result<PathBuf> {
    let lnk_path = dest.with_extension("lnk");
    let shell_link = mslnk::ShellLink::new(src)?;
    shell_link.create_lnk(&lnk_path)?;
    Ok(lnk_path)
}

#[cfg(target_os = "linux")]
pub fn create_shortcut(src: &Path, dest: &Path) -> Result<PathBuf> {
    let app_name = src
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "app".into());

    let desktop = format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name={name}\n\
         Exec=\"{exec}\"\n\
         Terminal=false\n",
        name = app_name,
        exec = src.to_string_lossy(),
    );

    let desktop_path = dest.with_extension("desktop");
    std::fs::write(&desktop_path, desktop)?;
    Ok(desktop_path)
}
