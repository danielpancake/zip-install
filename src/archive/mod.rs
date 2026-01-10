use std::path::Path;

use anyhow::Result;

use crate::models::ApplicationEntry;

pub trait Archive {
    // fn extract(&self, output_dir: &Path) -> Result<()>;
    fn list(&mut self) -> Vec<String>;

    fn candidates(&mut self) -> Vec<ApplicationEntry> {
        self.list()
            .into_iter()
            .filter(|e| e.to_ascii_lowercase().ends_with(".exe"))
            .map(|path| ApplicationEntry {
                name: path
                    .rsplit(&['/', '\\'][..])
                    .next()
                    .unwrap_or(&path)
                    .to_string(),
                path,
            })
            .collect()
    }
}

pub fn open_archive(path: &Path) -> Result<Box<dyn Archive>> {
    let archive = ZipArchiveHandler::open(path)?;
    Ok(Box::new(archive))
}

mod zip;

pub use zip::ZipArchiveHandler;
