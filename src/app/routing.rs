use crate::state::index::InstalledApp;

#[derive(Clone)]
pub enum Route {
    AppList,
    Setup,
    Install,
    Update(InstalledApp),
    ManualUpdate,
}

pub enum ViewAction {
    Navigate(Route),
    Back,
    Close,
}
