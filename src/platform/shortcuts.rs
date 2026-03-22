use std::path::Path;

use anyhow::Result;

#[cfg(target_os = "windows")]
pub fn create_shortcut(src: &Path, dest: &Path) -> Result<()> {
    let shell_link = mslnk::ShellLink::new(src)?;
    shell_link.create_lnk(dest.with_extension("lnk"))?;
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn create_shortcut(src: &Path, dest: &Path) -> Result<()> {
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

    std::fs::write(dest.with_extension("desktop"), desktop)?;
    Ok(())
}
