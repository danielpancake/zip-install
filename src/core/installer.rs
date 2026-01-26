use crate::core::models::ApplicationEntry;
use crate::package::Package;
use crate::platform::shortcuts::create_shortcut;
use crate::state::config::INSTALL_PATH;
use anyhow::Context;
use anyhow::Ok;
use anyhow::Result;

pub fn install(
    archive: &mut dyn Package,
    application: ApplicationEntry,
    create_desktop_shortcut: bool,
    create_start_menu_shortcut: bool,
) -> Result<()> {
    let local_dir =
        dirs::data_local_dir().context("Failed to get local application data directory")?;

    // TODO: this is bad, better management
    let uuid = uuid::Uuid::new_v4().to_string();
    let output_dir = local_dir.join(INSTALL_PATH).join(&uuid);

    archive
        .extract(std::path::Path::new(&output_dir))
        .context("Failed to extract package")?;

    let src_path = output_dir.join(application.path);

    if create_desktop_shortcut {
        let desktop_dir = dirs::desktop_dir().context("Failed to get desktop directory")?;
        let dest_path = desktop_dir.join(format!("{}.lnk", application.name));
        // TODO: .lnk is windows specific

        create_shortcut(src_path.as_path(), dest_path.as_path())
            .context("Failed to create desktop shortcut")?;
    }

    if create_start_menu_shortcut {
        return Ok(());
    }

    Ok(())
}
