use anyhow::Result;
use std::collections::HashSet;
use std::path::Path;

use crate::package::Package;

#[derive(Debug)]
pub struct Fingerprint {
    pub files: HashSet<String>,
    pub candidates: HashSet<String>,
}

impl Fingerprint {
    pub fn from_package(package: &dyn Package) -> Result<Self> {
        let files = package
            .list()
            .iter()
            .map(|e| Self::normalize_path(&e.to_string_lossy()))
            .collect();
        let candidates = package.candidates().iter().map(|e| e.app_name.clone()).collect();

        Ok(Self { files, candidates })
    }

    pub fn from_path(path: &Path) -> Result<Self> {
        let package = crate::package::DirPackage::open(path)?;
        Self::from_package(&package)
    }

    pub fn similarity(&self, other: &Self) -> f64 {
        let score_files = Self::jaccard(&self.files, &other.files);
        let score_candidates = Self::jaccard(&self.candidates, &other.candidates);

        (score_files + score_candidates) / 2.0
    }

    fn normalize_path(path: &str) -> String {
        path.replace('\\', "/")
            .split('/')
            .map(|component| crate::package::strip_version(Path::new(component)))
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
