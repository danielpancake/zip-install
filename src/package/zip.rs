use std::fs::File;
use std::path::{Path, PathBuf};

use anyhow::Result;
use zip::ZipArchive;

use crate::package::Package;

pub struct ZipArchiveHandler {
    path: PathBuf,
    zip: ZipArchive<File>,
}

impl ZipArchiveHandler {
    pub fn open(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let zip = ZipArchive::new(file)?;

        Ok(Self { path: path.into(), zip })
    }
}
impl Package for ZipArchiveHandler {
    fn extract(&mut self, output_dir: &Path) -> Result<PathBuf> {
        self.zip.extract(output_dir)?;
        Ok(output_dir.into())
    }

    #[cfg(windows)]
    fn is_executable(&self, path: &Path) -> bool {
        path.extension()
            .map(|ext| ext.eq_ignore_ascii_case("exe"))
            .unwrap_or(false)
    }

    #[cfg(unix)]
    // untested... does it even work? idk
    fn is_executable(&self, path: &Path) -> bool {
        self.zip
            .by_name(path.to_str().unwrap())
            .ok()
            .and_then(|f| f.unix_mode())
            .map(|mode| mode & 0o111 != 0)
            .unwrap_or(false)
    }

    fn list(&self) -> Vec<PathBuf> {
        self.zip
            .file_names()
            .filter(|e: &&str| !e.ends_with('/'))
            .map(|e| e.into())
            .collect()
    }

    fn source(&self) -> &Path {
        &self.path
    }
}
