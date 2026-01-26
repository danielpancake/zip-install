use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const CONFIG_DIR_NAME: &str = "zip-install";
const STATE_FILE_NAME: &str = "state.toml";

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InstallState {
    pub installed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_date: Option<String>,
}

impl InstallState {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&config_path).context("Failed to read config file")?;

        let state: InstallState =
            toml::from_str(&content).context("Failed to parse config file")?;

        Ok(state)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).context("Failed to create config directory")?;
        }

        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;

        fs::write(&config_path, content).context("Failed to write config file")?;

        Ok(())
    }

    pub fn mark_installed(&mut self) -> Result<()> {
        self.installed = true;
        self.install_date = Some(format!(
            "{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        ));
        self.save()
    }

    pub fn mark_uninstalled(&mut self) -> Result<()> {
        self.installed = false;
        self.install_date = None;
        self.save()
    }

    pub fn is_installed(&self) -> bool {
        self.installed
    }

    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir().context("Failed to get config directory")?;

        Ok(config_dir.join(CONFIG_DIR_NAME).join(STATE_FILE_NAME))
    }

    pub fn config_dir() -> Result<PathBuf> {
        let config_dir = dirs::config_dir().context("Failed to get config directory")?;

        Ok(config_dir.join(CONFIG_DIR_NAME))
    }
}
