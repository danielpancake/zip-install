use crate::package::Package;
use crate::state::config::Config;
use crate::state::index::InstalledApp;

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

    pub fn take(&mut self) -> Self {
        std::mem::replace(self, Self {
            candidates_index: 0,
            checkbox_shortcut_desktop: false,
            checkbox_shortcut_menu: false,
            checkbox_remove_package: false,
        })
    }
}

pub enum Route {
    AppList,
    SelfInstall,
    Install(Box<dyn Package>, SharedState),
    Update(Box<dyn Package>, InstalledApp, SharedState),
    ManualUpdate(Box<dyn Package>, SharedState),
}

pub enum ViewAction {
    Navigate(Route),
    Close,
}
