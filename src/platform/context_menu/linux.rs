use std::fs;
use std::path::PathBuf;

use anyhow::Result;

use super::ContextMenuItem;

fn service_menu_dir() -> Result<PathBuf> {
    let data_dir = dirs::data_dir().ok_or_else(|| anyhow::anyhow!("Failed to get data directory"))?;
    Ok(data_dir.join("kio").join("servicemenus"))
}

fn service_menu_path(app_name: &str) -> Result<PathBuf> {
    Ok(service_menu_dir()?.join(format!("{app_name}.desktop")))
}

pub fn add_context_menu(app_name: &str, item: &ContextMenuItem, extensions: &[&str]) -> Result<()> {
    let mime_types: Vec<String> = extensions.iter().filter_map(|ext| ext_to_mime(ext)).collect();

    let icon_line = item
        .icon_path
        .as_ref()
        .map(|p| format!("Icon={}", p.to_string_lossy()))
        .unwrap_or_default();

    let desktop = format!(
        "[Desktop Entry]\n\
         Type=Service\n\
         MimeType={mimes}\n\
         Actions={app_name}\n\
         \n\
         [Desktop Action {app_name}]\n\
         Name={label}\n\
         Exec=\"{exec}\" %f\n\
         {icon_line}",
        mimes = mime_types.join(";") + ";",
        label = item.label,
        exec = item.executable_path.to_string_lossy(),
    );

    let dir = service_menu_dir()?;
    fs::create_dir_all(&dir)?;
    fs::write(service_menu_path(app_name)?, desktop)?;

    Ok(())
}

pub fn remove_context_menu(app_name: &str, _extensions: &[&str]) -> Result<()> {
    let path = service_menu_path(app_name)?;
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

fn ext_to_mime(ext: &str) -> Option<String> {
    mime_guess::from_ext(ext.trim_start_matches('.'))
        .first()
        .map(|m| m.to_string())
}
