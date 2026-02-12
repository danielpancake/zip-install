use anyhow::{Context, Result};
use std::path::PathBuf;

const APP_DIR: &str = "ZipInstall";
const PACKAGES_DIR: &str = "Packages";
const INDEX_FILE: &str = "index.toml";
const CONFIG_FILE: &str = "config.toml";

pub fn app_dir() -> Result<PathBuf> {
    let main_dir = dirs::data_local_dir().context("Failed to get local data directory")?;
    Ok(main_dir.join(APP_DIR))
}

pub fn packages_dir() -> Result<PathBuf> {
    let app_dir = app_dir()?;
    Ok(app_dir.join(PACKAGES_DIR))
}

pub fn index_file() -> Result<PathBuf> {
    let app_dir = app_dir()?;
    Ok(app_dir.join(INDEX_FILE))
}

pub fn config_file() -> Result<PathBuf> {
    let app_dir = app_dir()?;
    Ok(app_dir.join(CONFIG_FILE))
}
