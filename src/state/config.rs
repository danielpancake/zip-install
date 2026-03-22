use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::state::paths;
use crate::state::persistable::Persistable;

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub is_installed: bool,

    pub default_shortcut_desktop: bool,
    pub default_shortcut_menu: bool,
    pub default_remove_package: bool,

    pub match_threshold: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            is_installed: false,

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
