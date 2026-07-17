pub mod bootstrap;
pub mod fingerprint;
pub mod installer;

use crate::core::fingerprint::Fingerprint;
use crate::state::config::Config;
use crate::state::index::{InstallIndex, InstalledApp, StoredFingerprint};
use crate::state::paths;
use crate::state::persistable::Persistable;

pub fn detect_update(index: &mut InstallIndex, fingerprint: &Fingerprint, config: &Config) -> Option<InstalledApp> {
    let packages_dir = paths::packages_dir().ok()?;

    let mut backfilled = false;
    let mut best: Option<(String, f64)> = None;

    for (uuid, app) in index.entries.iter_mut() {
        if Some(uuid.as_str()) == config.self_uuid.as_deref() {
            continue;
        }

        if app.fingerprint.is_empty() {
            let install_dir = packages_dir.join(uuid.as_str());
            if !install_dir.is_dir() {
                continue;
            }
            app.fingerprint = StoredFingerprint::from(&Fingerprint::from_path(&install_dir));
            backfilled = true;
        }

        let score = app.fingerprint.to_fingerprint().similarity(fingerprint);
        if score < config.match_threshold {
            continue;
        }

        let better = match &best {
            None => true,
            Some((best_uuid, best_score)) => {
                score > *best_score || (score == *best_score && uuid.as_str() > best_uuid.as_str())
            }
        };

        if better {
            best = Some((uuid.clone(), score));
        }
    }

    if backfilled {
        let _ = index.save();
    }

    let (uuid, _) = best?;
    let mut app = index.entries.get(&uuid).cloned()?;
    app.uuid = uuid;
    Some(app)
}
