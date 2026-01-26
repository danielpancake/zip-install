mod exe;
mod zip;

use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::core::models::ApplicationEntry;

pub use exe::StandaloneExecutable;
pub use zip::ZipArchiveHandler;

pub trait Package {
    fn candidates(&mut self) -> Vec<ApplicationEntry> {
        self.list()
            .into_iter()
            .filter(|e| self.is_executable(e))
            .map(ApplicationEntry::from)
            .collect()
    }
    fn extract(&mut self, output_dir: &Path) -> Result<PathBuf>;
    fn is_executable(&self, path: &Path) -> bool;
    fn list(&mut self) -> Vec<PathBuf>;
    fn source(&self) -> &Path;
}
pub fn open_package(path: &Path) -> Result<Box<dyn Package>> {
    let extension = path
        .extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    match extension.as_str() {
        "exe" => {
            let package = StandaloneExecutable::open(path)?;
            Ok(Box::new(package))
        }
        "zip" => {
            let package = ZipArchiveHandler::open(path)?;
            Ok(Box::new(package))
        }
        _ => Err(anyhow::anyhow!("Unsupported file format: .{}", extension)),
    }
}
