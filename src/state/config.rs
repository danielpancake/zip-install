use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::state::paths;
use crate::state::persistable::Persistable;

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub self_uuid: Option<String>,

    pub default_shortcut_desktop: bool,
    pub default_shortcut_menu: bool,
    pub default_remove_package: bool,

    pub match_threshold: f64,
}

impl Config {
    pub fn is_installed(&self) -> bool {
        self.self_uuid.is_some()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            self_uuid: None,

            default_shortcut_desktop: true,
            default_shortcut_menu: true,
            default_remove_package: false,

            match_threshold: 0.8,
        }
    }
}

impl Persistable for Config {
    fn path() -> Result<std::path::PathBuf> {
        paths::config_file()
    }
}
