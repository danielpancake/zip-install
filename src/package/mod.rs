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
    pub app_name: String,
    pub display_name: String,
    pub file_name: String,
    pub relative_path: PathBuf,
}

impl From<PathBuf> for Candidate {
    fn from(path: PathBuf) -> Self {
        let app_name = strip_version(&path);
        let file_name: String = path
            .file_name()
            .map(|name| name.to_string_lossy().into())
            .unwrap_or_default();

        Self {
            app_name,
            display_name: file_name.clone(),
            file_name,
            relative_path: path,
        }
    }
}

fn disambiguate_candidates(candidates: &mut Vec<Candidate>) {
    let mut counts = std::collections::HashMap::new();
    for c in candidates.iter() {
        *counts.entry(c.file_name.clone()).or_insert(0usize) += 1;
    }

    for c in candidates.iter_mut() {
        if counts[&c.file_name] > 1 {
            if let Some(parent) = c.relative_path.parent() {
                let folder = parent
                    .components()
                    .last()
                    .map(|comp| comp.as_os_str().to_string_lossy().into_owned())
                    .unwrap_or_default();

                if !folder.is_empty() {
                    c.display_name = format!("{} ({})", c.file_name, folder);
                }
            }
        }
    }
}

pub trait Package {
    fn candidates(&self) -> Vec<Candidate> {
        let mut candidates: Vec<Candidate> = self
            .list()
            .into_iter()
            .filter(|e| self.is_executable(e))
            .map(Candidate::from)
            .collect();

        disambiguate_candidates(&mut candidates);
        candidates
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
        Regex::new(r"[-_]?v?\d+([.-]\d+)+([-_]?(x64|x86|win64|win32))?([.]exe)?$").expect("Invalid regex pattern")
    });

    let name = path.file_name().unwrap_or(path.as_os_str()).to_string_lossy();

    re.replace(&name, "").into()
}
