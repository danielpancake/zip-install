use crate::package::{Candidate, Package};
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
}

pub struct ViewContext<'a> {
    pub package: &'a mut dyn Package,
    pub shared: &'a mut SharedState,
    pub candidates: &'a [Candidate],
}

pub enum Route {
    AppList,
    SelfInstall,
    Install,
    Update(InstalledApp),
    ManualUpdate,
}

pub enum ViewAction {
    Navigate(Route),
    Back,
    Close,
}
