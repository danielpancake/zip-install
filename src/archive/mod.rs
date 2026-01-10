use std::path::Path;

use anyhow::Result;

use crate::models::ApplicationEntry;

pub trait Archive {
    fn candidates(&mut self) -> Vec<ApplicationEntry> {
        self.list()
            .into_iter()
            // TODO: make platform-specific
            .filter(|e| e.to_ascii_lowercase().ends_with(".exe"))
            .map(|path| {
                let path_obj = Path::new(&path);
                let name = path_obj
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or(&path)
                    .to_string();

                ApplicationEntry { name, path }
            })
            .collect()
    }

    fn extract(&mut self, output_dir: &Path) -> Result<()>;

    fn list(&mut self) -> Vec<String>;
}

pub fn open_archive(path: &Path) -> Result<Box<dyn Archive>> {
    let archive = ZipArchiveHandler::open(path)?;
    Ok(Box::new(archive))
}

mod zip;

pub use zip::ZipArchiveHandler;
