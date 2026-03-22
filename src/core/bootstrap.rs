use anyhow::{Context, Result};
use std::fs;

use crate::package::Candidate;
use crate::platform::context_menu::{self, ContextMenuItem};
use crate::state::config::Config;
use crate::state::index::{InstallIndex, InstalledApp};
use crate::state::paths;
use crate::state::persistable::Persistable;

pub const EXTENSIONS: &[&str] = &[".zip", ".exe"]; // ".7z", ".rar"
pub const APP_NAME: &str = "ZipInstall";
pub const MENU_LABEL: &str = "Install with zip-install";

#[cfg(target_os = "windows")]
pub const EXE_NAME: &str = "zip-install.exe";

#[cfg(target_os = "linux")]
pub const EXE_NAME: &str = "zip-install";

pub fn setup() -> Result<()> {
    let packages_dir = paths::packages_dir()?;
    fs::create_dir_all(&packages_dir).context("Failed to create packages directory")?;

    let uuid = uuid::Uuid::new_v4().to_string();
    let install_dir = packages_dir.join(&uuid);
    fs::create_dir_all(&install_dir).context("Failed to create install directory")?;

    let current_exe = std::env::current_exe().context("Failed to get current executable path")?;
    let dest_exe = install_dir.join(EXE_NAME);

    fs::copy(&current_exe, &dest_exe).context("Failed to copy executable")?;

    let menu_item = ContextMenuItem::new(MENU_LABEL, &dest_exe).with_icon_path(&dest_exe);
    context_menu::add_context_menu(APP_NAME, &menu_item, EXTENSIONS)?;

    let candidate = Candidate::from(std::path::PathBuf::from(EXE_NAME));
    let mut index = InstallIndex::load().unwrap_or_default();
    index.add_entry(&uuid, InstalledApp::from(&candidate));
    index.save()?;

    let mut config = Config::load().unwrap_or_default();
    config.self_uuid = Some(uuid);
    config.save()?;

    Ok(())
}

pub fn uninstall() -> Result<()> {
    context_menu::remove_context_menu(APP_NAME, EXTENSIONS)?;

    let mut config = Config::load().unwrap_or_default();

    if let Some(uuid) = config.self_uuid.take() {
        let package_dir = paths::packages_dir()?.join(&uuid);
        if package_dir.exists() {
            fs::remove_dir_all(&package_dir).context("Failed to remove package directory")?;
        }

        let mut index = InstallIndex::load().unwrap_or_default();
        index.remove_entry(&uuid);
        index.save()?;
    }

    config.save()?;

    Ok(())
}
