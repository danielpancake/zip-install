use std::{fs::File, path::Path};

use crate::archive::Archive;
use anyhow::Result;
use zip::ZipArchive;

pub struct ZipArchiveHandler {
    zip: ZipArchive<File>,
}

impl ZipArchiveHandler {
    pub fn open(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let zip = ZipArchive::new(file)?;

        Ok(Self { zip })
    }
}

impl Archive for ZipArchiveHandler {
    fn extract(&mut self, output_dir: &Path) -> Result<()> {
        self.zip.extract(output_dir)?;
        Ok(())
    }

    fn list(&mut self) -> Vec<String> {
        self.zip.file_names().map(|name| name.to_string()).collect()
    }
}
