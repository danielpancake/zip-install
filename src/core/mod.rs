pub mod bootstrap;
pub mod fingerprint;
pub mod installer;

use crate::core::fingerprint::{AppMatcher, Fingerprint};
use crate::package::Package;
use crate::state::config::Config;
use crate::state::index::{InstallIndex, InstalledApp};
use crate::state::paths;
use crate::state::persistable::Persistable;

pub fn detect_update(package: &dyn Package, config: &Config) -> Option<InstalledApp> {
    let fingerprint = Fingerprint::from_package(package).ok()?;

    let packages_dir = paths::packages_dir().ok()?;
    let mut matcher = AppMatcher::new();
    matcher.scan_installations(packages_dir.as_path()).ok()?;

    let (path, _score) = matcher.find_match(&fingerprint, config.match_threshold)?;

    let uuid = path.file_name()?.to_string_lossy().into_owned();
    let index = InstallIndex::load().unwrap_or_default();

    let mut app = index.entries.get(&uuid).cloned()?;
    app.uuid = uuid;
    Some(app)
}
