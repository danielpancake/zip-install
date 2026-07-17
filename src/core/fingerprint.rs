use std::collections::HashSet;
use std::path::Path;

use crate::package::{DirPackage, Package, strip_version};
use crate::state::index::StoredFingerprint;

#[derive(Debug)]
pub struct Fingerprint {
    pub files: HashSet<String>,
    pub candidates: HashSet<String>,
}

impl Fingerprint {
    pub fn from_package(package: &dyn Package) -> Self {
        let entries = package.list();

        let candidates = entries
            .iter()
            .filter(|path| package.is_executable(path))
            .map(|path| strip_version(path))
            .collect();

        let files = entries
            .iter()
            .map(|e| Self::normalize_path(&e.to_string_lossy()))
            .collect();

        Self { files, candidates }
    }

    pub fn from_path(path: &Path) -> Self {
        Self::from_package(&DirPackage::open(path))
    }

    pub fn similarity(&self, other: &Self) -> f64 {
        let score_files = Self::jaccard(&self.files, &other.files);
        let score_candidates = Self::jaccard(&self.candidates, &other.candidates);

        (score_files + score_candidates) / 2.0
    }

    fn normalize_path(path: &str) -> String {
        path.replace('\\', "/")
            .split('/')
            .map(|component| strip_version(Path::new(component)))
            .collect::<Vec<_>>()
            .join("/")
    }

    fn jaccard(a: &HashSet<String>, b: &HashSet<String>) -> f64 {
        let isect = a.intersection(b).count();
        let union = a.union(b).count();

        if union == 0 {
            return 0.0;
        }

        isect as f64 / union as f64
    }
}

impl From<&Fingerprint> for StoredFingerprint {
    fn from(fingerprint: &Fingerprint) -> Self {
        let mut files: Vec<String> = fingerprint.files.iter().cloned().collect();
        let mut candidates: Vec<String> = fingerprint.candidates.iter().cloned().collect();
        files.sort();
        candidates.sort();

        Self { files, candidates }
    }
}

impl StoredFingerprint {
    pub fn to_fingerprint(&self) -> Fingerprint {
        Fingerprint {
            files: self.files.iter().cloned().collect(),
            candidates: self.candidates.iter().cloned().collect(),
        }
    }
}
