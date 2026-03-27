#[cfg(unix)]
use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};

use anyhow::Result;
use flate2::read::GzDecoder;
use tar::Archive;

use crate::package::Package;

pub struct TarGzArchiveHandler {
    path: PathBuf,
    entries: Vec<PathBuf>,
    #[cfg(unix)]
    unix_modes: HashMap<PathBuf, u32>,
}

impl TarGzArchiveHandler {
    pub fn open(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let decoder = GzDecoder::new(file);
        let mut archive = Archive::new(decoder);

        let mut entries = Vec::new();
        #[cfg(unix)]
        let mut unix_modes = HashMap::new();

        for entry in archive.entries()? {
            let entry = entry?;
            let entry_path = entry.path()?.into_owned();

            if entry.header().entry_type().is_file() {
                #[cfg(unix)]
                if let Ok(mode) = entry.header().mode() {
                    unix_modes.insert(entry_path.clone(), mode);
                }

                entries.push(entry_path);
            }
        }

        Ok(Self {
            path: path.into(),
            entries,
            #[cfg(unix)]
            unix_modes,
        })
    }
}

impl Package for TarGzArchiveHandler {
    fn extract(&mut self, output_dir: &Path) -> Result<PathBuf> {
        let file = File::open(&self.path)?;
        let decoder = GzDecoder::new(file);
        let mut archive = Archive::new(decoder);
        archive.unpack(output_dir)?;
        Ok(output_dir.into())
    }

    #[cfg(windows)]
    fn is_executable(&self, path: &Path) -> bool {
        path.extension()
            .map(|ext| ext.eq_ignore_ascii_case("exe"))
            .unwrap_or(false)
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
