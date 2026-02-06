use crate::package::Package;

pub enum Route {
    AppList,
    SelfInstall,
    ZipInstall(Box<dyn Package>),
}

pub enum ViewAction {
    Navigate(Route),
    Close,
}
