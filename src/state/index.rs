use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::package::Candidate;
use crate::state::paths;
use crate::state::persistable::Persistable;

#[derive(Clone, Serialize, Deserialize)]
pub struct InstalledApp {
    #[serde(default)]
    pub uuid: String,
    pub app_name: String,
    pub file_name: String,
    pub main_path: String,
    pub installed_at: String,
}

impl From<&Candidate> for InstalledApp {
    fn from(candidate: &Candidate) -> Self {
        Self {
            uuid: String::new(),
            app_name: candidate.app_name.clone(),
            file_name: candidate.file_name.clone(),
            main_path: candidate.relative_path.to_string_lossy().into_owned(),
            installed_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
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
}

impl Persistable for InstallIndex {
    fn path() -> Result<std::path::PathBuf> {
        paths::index_file()
    }
}
