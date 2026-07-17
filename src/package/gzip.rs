use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};

use anyhow::Result;
use flate2::read::GzDecoder;

use super::Package;

pub struct GzipStandalone {
    path: PathBuf,
    inner_name: PathBuf,
}

impl GzipStandalone {
    pub fn open(path: &Path) -> Result<Self> {
        let file_name = path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("Invalid file path"))?
            .to_string_lossy();

        let stripped = file_name
            .strip_suffix(".gz")
            .or_else(|| file_name.strip_suffix(".GZ"))
            .unwrap_or(&file_name);

        let inner_name = PathBuf::from(stripped);

        Ok(Self {
            path: path.into(),
            inner_name,
        })
    }
}

impl Package for GzipStandalone {
    fn extract(&mut self, output_dir: &Path) -> Result<PathBuf> {
        fs::create_dir_all(output_dir)?;

        let dest_path = output_dir.join(&self.inner_name);
        let input = File::open(&self.path)?;
        let mut decoder = GzDecoder::new(input);
        let mut output = File::create(&dest_path)?;
        io::copy(&mut decoder, &mut output)?;

        Ok(dest_path)
    }

    #[cfg(windows)]
    fn is_executable(&self, path: &Path) -> bool {
        crate::package::has_exe_extension(path)
    }

    #[cfg(unix)]
    fn is_executable(&self, _path: &Path) -> bool {
        true
    }

    fn list(&self) -> Vec<PathBuf> {
        vec![self.inner_name.clone()]
    }

    fn source(&self) -> &Path {
        &self.path
    }
}
