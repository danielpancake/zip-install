use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::archive::Archive;
use anyhow::Result;

pub struct SingleExeHandler {
    path: PathBuf,
}

impl SingleExeHandler {
    pub fn open(path: &Path) -> Result<Self> {
        Ok(Self {
            path: path.to_path_buf(),
        })
    }
}

impl Archive for SingleExeHandler {
    fn extract(&mut self, output_dir: &Path) -> Result<()> {
        // Create output directory if it doesn't exist
        fs::create_dir_all(output_dir)?;

        // Get the file name
        let file_name = self
            .path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("Invalid file path"))?;

        // Copy the exe to the output directory
        let dest_path = output_dir.join(file_name);
        fs::copy(&self.path, &dest_path)?;

        Ok(())
    }

    fn list(&mut self) -> Vec<String> {
        // Return just the file name
        vec![
            self.path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string(),
        ]
    }
}
