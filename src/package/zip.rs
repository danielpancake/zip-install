#[cfg(unix)]
use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};

use anyhow::Result;
use zip::ZipArchive;

use crate::package::Package;

pub struct ZipArchiveHandler {
    path: PathBuf,
    zip: ZipArchive<File>,
    entries: Vec<PathBuf>,
    #[cfg(unix)]
    unix_modes: HashMap<PathBuf, u32>,
}

impl ZipArchiveHandler {
    pub fn open(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let mut zip = ZipArchive::new(file)?;

        let mut entries = Vec::new();
        #[cfg(unix)]
        let mut unix_modes = HashMap::new();

        for i in 0..zip.len() {
            let entry = zip.by_index_raw(i)?;
            if entry.is_dir() {
                continue;
            }

            let entry_path = PathBuf::from(entry.name());

            #[cfg(unix)]
            if let Some(mode) = entry.unix_mode() {
                unix_modes.insert(entry_path.clone(), mode);
            }

            entries.push(entry_path);
        }

        Ok(Self {
            path: path.into(),
            zip,
            entries,
            #[cfg(unix)]
            unix_modes,
        })
    }
}

impl Package for ZipArchiveHandler {
    fn extract(&mut self, output_dir: &Path) -> Result<PathBuf> {
        self.zip.extract(output_dir)?;
        Ok(output_dir.into())
    }

    #[cfg(windows)]
    fn is_executable(&self, path: &Path) -> bool {
        crate::package::has_exe_extension(path)
    }

    #[cfg(unix)]
    fn is_executable(&self, path: &Path) -> bool {
        self.unix_modes.get(path).map(|mode| mode & 0o111 != 0).unwrap_or(false)
    }

    fn list(&self) -> Vec<PathBuf> {
        self.entries.clone()
    }

    fn source(&self) -> &Path {
        &self.path
    }
}
