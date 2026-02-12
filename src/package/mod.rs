mod exe;
mod zip;

use std::path::{Path, PathBuf};

use anyhow::Result;

pub use exe::StandaloneExecutable;
pub use zip::ZipArchiveHandler;

#[derive(Clone)]
pub struct PackageEntry {
    pub name: String,
    pub path: PathBuf,
}

impl From<PathBuf> for PackageEntry {
    fn from(path: PathBuf) -> Self {
        let name = path
            .file_name()
            .map(|name| name.to_string_lossy().into())
            .unwrap_or_default();

        Self { name, path }
    }
}

pub trait Package {
    fn candidates(&self) -> Vec<PackageEntry> {
        self.list()
            .into_iter()
            .filter(|e| self.is_executable(e))
            .map(PackageEntry::from)
            .collect()
    }

    fn extract(&mut self, output_dir: &Path) -> Result<PathBuf>;

    fn is_executable(&self, path: &Path) -> bool;

    fn list(&self) -> Vec<PathBuf>;

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
