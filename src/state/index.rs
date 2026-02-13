use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::fingerprint::Fingerprint;
use crate::package::Candidate;
use crate::state::paths;
use crate::state::persistable::Persistable;

#[derive(Clone, Serialize, Deserialize)]
pub struct InstalledApp {
    pub file_name: String,
    pub base_name: String,
    pub main_path: String,
    pub installed_at: String,
}

impl From<&Candidate> for InstalledApp {
    fn from(candidate: &Candidate) -> Self {
        Self {
            file_name: candidate.file_name.clone(),
            base_name: candidate.base_name.clone(),
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
    pub fn add_entry(&mut self, uuid: &str, entry: InstalledApp) {
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

pub struct AppMatcher {
    pub known_apps: HashMap<PathBuf, Fingerprint>,
}

impl AppMatcher {
    pub fn new() -> Self {
        Self {
            known_apps: HashMap::new(),
        }
    }

    pub fn scan_installations(&mut self, install_dir: &Path) -> Result<()> {
        if !install_dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(install_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir()
                && let Ok(fingerprint) = Fingerprint::from_path(&path)
            {
                self.known_apps.insert(path, fingerprint);
            }
        }

        Ok(())
    }

    pub fn find_match(&self, archive_fingerprint: &Fingerprint, threshold: f64) -> Option<(PathBuf, f64)> {
        self.known_apps
            .iter()
            .map(|(path, fp)| (path.clone(), fp.similarity(archive_fingerprint)))
            .filter(|(_, score)| *score >= threshold)
            .max_by(|(_, score1), (_, score2)| score1.partial_cmp(score2).unwrap_or(std::cmp::Ordering::Equal))
    }
}
