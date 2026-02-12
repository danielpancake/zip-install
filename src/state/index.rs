use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::state::paths;
use crate::state::persistable::Persistable;

#[derive(Serialize, Deserialize)]
pub struct InstallEntry {
    pub name: String,
    pub main_executable: String,
    pub uuid: String,
    pub date: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct InstallIndex {
    pub entries: HashMap<String, InstallEntry>,
}

impl Persistable for InstallIndex {
    fn path() -> Result<std::path::PathBuf> {
        paths::index_file()
    }
}

impl InstallIndex {
    pub fn add_entry(&mut self, entry: InstallEntry) {
        self.entries.insert(entry.uuid.clone(), entry);
    }

    pub fn remove_entry(&mut self, uuid: &str) {
        self.entries.remove(uuid);
    }
}
