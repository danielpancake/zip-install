use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::package::Package;

pub struct DirPackage {
    path: PathBuf,
}

impl DirPackage {
    pub fn open(path: &Path) -> Result<Self> {
        Ok(Self { path: path.into() })
    }

    fn collect_files(&self, dir: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for entry in fs::read_dir(dir)? {
            let path = entry?.path();

            if path.is_dir() {
                files.extend(self.collect_files(&path)?);
            } else if let Ok(rel) = path.strip_prefix(&self.path) {
                let normalized = rel.iter().collect::<PathBuf>();
                files.push(PathBuf::from(normalized.to_string_lossy().replace('\\', "/")));
            }
        }
        Ok(files)
    }
}

impl Package for DirPackage {
    fn extract(&mut self, output_dir: &Path) -> Result<PathBuf> {
        if output_dir == self.path {
            return Ok(self.path.clone());
        }

        fs::create_dir_all(output_dir)?;

        fs_extra::dir::copy(
            &self.path,
            output_dir,
            &fs_extra::dir::CopyOptions {
                overwrite: true,
                content_only: true,
                ..Default::default()
            },
        )
        .context("Failed to copy directory content")?;

        Ok(output_dir.to_path_buf())
    }

    fn is_executable(&self, path: &Path) -> bool {
        #[cfg(windows)]
        {
            path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("exe"))
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            self.path
                .join(path)
                .metadata()
                .map(|m| m.permissions().mode() & 0o111 != 0)
                .unwrap_or(false)
        }
    }

    fn list(&self) -> Vec<PathBuf> {
        self.collect_files(&self.path).unwrap_or_default()
    }

    fn source(&self) -> &Path {
        &self.path
    }
}
