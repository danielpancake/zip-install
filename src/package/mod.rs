mod dir;
mod exe;
mod zip;

use std::path::{Path, PathBuf};

use anyhow::Result;
use regex::Regex;
use std::sync::OnceLock;

pub use dir::DirPackage;
pub use exe::StandaloneExecutable;
pub use zip::ZipArchiveHandler;

#[derive(Clone)]
pub struct Candidate {
    pub file_name: String,
    pub base_name: String,
    pub relative_path: PathBuf,
}

impl From<PathBuf> for Candidate {
    fn from(path: PathBuf) -> Self {
        let file_name = path
            .file_name()
            .map(|name| name.to_string_lossy().into())
            .unwrap_or_default();

        let base_name = strip_version(&path);

        Self {
            file_name,
            base_name,
            relative_path: path,
        }
    }
}

pub trait Package {
    fn candidates(&self) -> Vec<Candidate> {
        self.list()
            .into_iter()
            .filter(|e| self.is_executable(e))
            .map(Candidate::from)
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

pub fn strip_version(path: &Path) -> String {
    static VERSION_REGEX: OnceLock<Regex> = OnceLock::new();

    let re = VERSION_REGEX.get_or_init(|| {
        Regex::new(r"[-_]?v?\d+([.-]\d+)*[-_]?(x64|x86|win64|win32)?$").expect("Invalid regex pattern")
    });

    let name = path.file_name().unwrap_or(path.as_os_str()).to_string_lossy();

    re.replace(&name, "").into()
}
