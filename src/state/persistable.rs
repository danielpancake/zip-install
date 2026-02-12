use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Serialize, de::DeserializeOwned};

pub trait Persistable: Default + Serialize + DeserializeOwned {
    fn path() -> Result<PathBuf>;

    fn load() -> Result<Self> {
        let path = Self::path()?;

        if !path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(&path).with_context(|| format!("Failed to read file: {:?}", path))?;

        toml::from_str(&content).with_context(|| format!("Failed to parse file: {:?}", path))
    }

    fn save(&self) -> Result<()> {
        let path = Self::path()?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).with_context(|| format!("Failed to create directory: {:?}", parent))?;
        }

        let content = toml::to_string_pretty(self).context("Failed to serialize data")?;

        std::fs::write(&path, content).with_context(|| format!("Failed to write file: {:?}", path))
    }
}
