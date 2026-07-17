use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use regex_lite::Regex;

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

pub fn disambiguate_candidates(candidates: &mut [Candidate]) {
    let mut counts = std::collections::HashMap::new();
    for c in candidates.iter() {
        *counts.entry(c.file_name.clone()).or_insert(0usize) += 1;
    }

    for c in candidates.iter_mut() {
        if counts[&c.file_name] > 1
            && let Some(parent) = c.relative_path.parent()
        {
            let folder = parent
                .components()
                .next_back()
                .map(|comp| comp.as_os_str().to_string_lossy().into_owned())
                .unwrap_or_default();

            if !folder.is_empty() {
                c.display_name = format!("{} ({})", c.file_name, folder);
            }
        }
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
