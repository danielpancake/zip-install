use anyhow::{Context, Ok, Result};

use crate::package::{Candidate, Package};
use crate::platform::shortcuts::create_shortcut;
use crate::state::paths::{applications_dir, packages_dir};

pub fn install(
    archive: &mut dyn Package,
    application: Candidate,
    create_desktop_shortcut: bool,
    create_app_launcher_shortcut: bool,
) -> Result<String> {
    let uuid = uuid::Uuid::new_v4().to_string();

    let output_dir = packages_dir()?.join(&uuid);
    let src_path = output_dir.join(&application.relative_path);

    archive
        .extract(output_dir.as_path())
        .context("Failed to extract package")?;

    if create_desktop_shortcut {
        let desktop_dir = dirs::desktop_dir().context("Failed to get desktop directory")?;
        let dest_path = desktop_dir.join(&application.file_name);
        create_shortcut(src_path.as_path(), dest_path.as_path()).context("Failed to create desktop shortcut")?;
    }

    if create_app_launcher_shortcut {
        let dest_path = applications_dir()?.join(&application.file_name);
        create_shortcut(src_path.as_path(), dest_path.as_path()).context("Failed to create app launcher shortcut")?;
    }

    Ok(uuid)
}

pub fn update(
    archive: &mut dyn Package,
    application: Candidate,
    target_uuid: &str,
    create_desktop_shortcut: bool,
    create_app_launcher_shortcut: bool,
) -> Result<()> {
    let output_dir = packages_dir()?.join(target_uuid);
    let src_path = output_dir.join(&application.relative_path);

    if output_dir.exists() {
        std::fs::remove_dir_all(&output_dir).context("Failed to remove old installation")?;
    }

    archive
        .extract(output_dir.as_path())
        .context("Failed to extract package")?;

    if create_desktop_shortcut {
        let desktop_dir = dirs::desktop_dir().context("Failed to get desktop directory")?;
        let dest_path = desktop_dir.join(&application.file_name);
        create_shortcut(src_path.as_path(), dest_path.as_path()).context("Failed to create desktop shortcut")?;
    }

    if create_app_launcher_shortcut {
        let dest_path = applications_dir()?.join(&application.file_name);
        create_shortcut(src_path.as_path(), dest_path.as_path()).context("Failed to create app launcher shortcut")?;
    }

    Ok(())
}
