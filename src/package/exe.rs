use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;

use super::Package;

pub struct StandaloneExecutable {
    path: PathBuf,
}

impl StandaloneExecutable {
    pub fn open(path: &Path) -> Result<Self> {
        Ok(Self { path: path.into() })
    }
}

impl Package for StandaloneExecutable {
    fn extract(&mut self, output_dir: &Path) -> Result<PathBuf> {
        fs::create_dir_all(output_dir)?;

        let file_name = self
            .path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("Invalid file path"))?;

        let dest_path = output_dir.join(file_name);
        fs::copy(&self.path, &dest_path)?;

        Ok(dest_path)
    }

    fn is_executable(&self, _path: &Path) -> bool {
        true
    }

    fn list(&self) -> Vec<PathBuf> {
        let file_name = self.path.file_name().map(PathBuf::from).unwrap_or_default();
        vec![file_name]
    }

    fn source(&self) -> &Path {
        &self.path
    }
}
