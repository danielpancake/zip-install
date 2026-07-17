use anyhow::{Context, Result};
use std::path::PathBuf;

use crate::package::{Candidate, Package};
use crate::platform::shortcuts::create_shortcut;
use crate::state::paths::{applications_dir, packages_dir};

pub fn install(
    package: &mut dyn Package,
    application: &Candidate,
    target_uuid: Option<&str>,
    old_shortcuts: &[PathBuf],
    create_desktop_shortcut: bool,
    create_app_launcher_shortcut: bool,
) -> Result<(String, Vec<PathBuf>)> {
    let uuid = target_uuid
        .map(str::to_string)
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    let output_dir = packages_dir()?.join(&uuid);
    let src_path = output_dir.join(&application.relative_path);

    if target_uuid.is_some() && output_dir.exists() {
        std::fs::remove_dir_all(&output_dir).context("Failed to remove old installation")?;
    }

    for shortcut in old_shortcuts {
        let _ = std::fs::remove_file(shortcut);
    }

    package
        .extract(output_dir.as_path())
        .context("Failed to extract package")?;

    let mut shortcuts = Vec::new();

    if create_desktop_shortcut {
        let desktop_dir = dirs::desktop_dir().context("Failed to get desktop directory")?;
        let dest_path = desktop_dir.join(&application.file_name);
        shortcuts.push(create_shortcut(&src_path, &dest_path).context("Failed to create desktop shortcut")?);
    }

    if create_app_launcher_shortcut {
        let dest_path = applications_dir()?.join(&application.file_name);
        shortcuts.push(create_shortcut(&src_path, &dest_path).context("Failed to create app launcher shortcut")?);
    }

    Ok((uuid, shortcuts))
}
