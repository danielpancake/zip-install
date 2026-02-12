use crate::package::Package;
use crate::state::config::Config;

pub enum Route {
    AppList,
    SelfInstall,
    ZipInstall(Box<dyn Package>, Config),
}

pub enum ViewAction {
    Navigate(Route),
    Close,
}
