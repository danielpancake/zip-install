use anyhow::{Context, Ok, Result};

use crate::package::{Candidate, Package};
use crate::platform::shortcuts::create_shortcut;
use crate::state::paths::packages_dir;

pub fn install(
    archive: &mut dyn Package,
    application: Candidate,
    create_desktop_shortcut: bool,
    create_start_menu_shortcut: bool,
) -> Result<String> {
    // TODO: this is bad, better management
    let uuid = uuid::Uuid::new_v4().to_string();
    let output_dir = packages_dir()?.join(&uuid);

    archive
        .extract(output_dir.as_path())
        .context("Failed to extract package")?;

    let src_path = output_dir.join(&application.relative_path);
    if create_desktop_shortcut {
        let desktop_dir = dirs::desktop_dir().context("Failed to get desktop directory")?;
        let dest_path = desktop_dir.join(format!("{}.lnk", application.file_name));
        // TODO: .lnk is windows specific
        create_shortcut(src_path.as_path(), dest_path.as_path()).context("Failed to create desktop shortcut")?;
    }

    // if create_start_menu_shortcut {}

    Ok(uuid)
}
