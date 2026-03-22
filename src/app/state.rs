use crate::package::{Candidate, Package};
use crate::state::config::Config;

pub struct AppData {
    pub package: Option<Box<dyn Package>>,
    pub shared: SharedState,
    pub candidates: Vec<Candidate>,
    pub is_installed: bool,
}

pub struct SharedState {
    pub candidates_index: usize,

    pub checkbox_shortcut_desktop: bool,
    pub checkbox_shortcut_menu: bool,
    pub checkbox_remove_package: bool,
}

impl SharedState {
    pub fn from_config(config: &Config) -> Self {
        Self {
            candidates_index: 0,

            checkbox_shortcut_desktop: config.default_shortcut_desktop,
            checkbox_shortcut_menu: config.default_shortcut_menu,
            checkbox_remove_package: config.default_remove_package,
        }
    }
}
