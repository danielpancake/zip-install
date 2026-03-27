use std::path::{Path, PathBuf};

use anyhow::Result;
use sevenz_rust::{Password, SevenZReader};

use crate::package::Package;

pub struct SevenZArchiveHandler {
    path: PathBuf,
    entries: Vec<PathBuf>,
}

impl SevenZArchiveHandler {
    pub fn open(path: &Path) -> Result<Self> {
        let mut entries = Vec::new();
        let mut reader = SevenZReader::open(path, Password::empty())?;

        reader.for_each_entries(|entry, _| {
            if !entry.is_directory() {
                entries.push(PathBuf::from(entry.name()));
            }
            Ok(true)
        })?;

        Ok(Self {
            path: path.into(),
            entries,
        })
    }
}

impl Package for SevenZArchiveHandler {
    fn extract(&mut self, output_dir: &Path) -> Result<PathBuf> {
        sevenz_rust::decompress_file(&self.path, output_dir)?;
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
        // sevenz-rust doesn't expose unix mode bits, so use a name-based heuristic:
        // files with no extension under a path containing "bin" are likely executables
        let has_no_extension = path.extension().is_none();
        let under_bin = path.components().any(|c| c.as_os_str() == "bin");
        has_no_extension && under_bin
    }

    fn list(&self) -> Vec<PathBuf> {
        self.entries.clone()
    }

    fn source(&self) -> &Path {
        &self.path
    }
}
