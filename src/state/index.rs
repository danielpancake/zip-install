use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::package::Candidate;
use crate::state::paths;
use crate::state::persistable::Persistable;

/// Install-time snapshot of a package's contents, kept in the index so
/// update detection doesn't have to re-walk every installed directory.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct StoredFingerprint {
    #[serde(default)]
    pub files: Vec<String>,
    #[serde(default)]
    pub candidates: Vec<String>,
}

impl StoredFingerprint {
    pub fn is_empty(&self) -> bool {
        self.files.is_empty() && self.candidates.is_empty()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InstalledApp {
    #[serde(skip)]
    pub uuid: String,
    pub app_name: String,
    pub file_name: String,
    pub main_path: String,
    pub installed_at: String,

    /// Shortcut files created for this app, so removal/update can delete
    /// exactly what was created and nothing else.
    #[serde(default)]
    pub shortcuts: Vec<String>,

    // Keep last: nested tables must follow plain values in TOML.
    #[serde(default)]
    pub fingerprint: StoredFingerprint,
}

impl From<&Candidate> for InstalledApp {
    fn from(candidate: &Candidate) -> Self {
        Self {
            uuid: String::new(),
            app_name: candidate.app_name.clone(),
            file_name: candidate.file_name.clone(),
            main_path: candidate.relative_path.to_string_lossy().into_owned(),
            installed_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            shortcuts: Vec::new(),
            fingerprint: StoredFingerprint::default(),
        }
    }
}

impl InstalledApp {
    /// Index of the candidate matching this app's recorded executable. An
    /// exact relative-path match wins; otherwise the first candidate with the
    /// same file name (the path may move between versioned folders).
    pub fn matching_candidate(&self, candidates: &[Candidate]) -> Option<usize> {
        candidates
            .iter()
            .position(|c| c.relative_path == Path::new(&self.main_path))
            .or_else(|| candidates.iter().position(|c| c.file_name == self.file_name))
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct InstallIndex {
    pub entries: HashMap<String, InstalledApp>,
}

impl InstallIndex {
    pub fn add_entry(&mut self, uuid: &str, mut entry: InstalledApp) {
        entry.uuid = uuid.to_string();
        self.entries.insert(uuid.to_string(), entry);
    }

    pub fn remove_entry(&mut self, uuid: &str) {
        self.entries.remove(uuid);
    }

    /// Installed apps with `uuid` filled in, minus `exclude` (zip-install's
    /// own entry), sorted by name for stable UI lists.
    pub fn apps(&self, exclude: Option<&str>) -> Vec<InstalledApp> {
        let mut apps: Vec<InstalledApp> = self
            .entries
            .iter()
            .filter(|(uuid, _)| Some(uuid.as_str()) != exclude)
            .map(|(uuid, app)| {
                let mut app = app.clone();
                app.uuid = uuid.clone();
                app
            })
            .collect();

        apps.sort_by(|a, b| a.app_name.cmp(&b.app_name).then_with(|| a.uuid.cmp(&b.uuid)));
        apps
    }
}

impl Persistable for InstallIndex {
    fn path() -> Result<std::path::PathBuf> {
        paths::index_file()
    }
}
