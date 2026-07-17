use std::path::{Component, Path, PathBuf};

use anyhow::Result;
use sevenz_rust::{Password, SevenZReader};

use crate::package::Package;

fn sanitized_join(base: &Path, entry_name: &str) -> Option<PathBuf> {
    let mut out = base.to_path_buf();
    for component in Path::new(entry_name).components() {
        match component {
            Component::Normal(part) => out.push(part),
            Component::CurDir => {}
            _ => return None,
        }
    }
    Some(out)
}

pub struct SevenZArchiveHandler {
    path: PathBuf,
    entries: Vec<PathBuf>,
}

impl SevenZArchiveHandler {
    pub fn open(path: &Path) -> Result<Self> {
        let mut entries = Vec::new();
        let mut reader = SevenZReader::open(path, Password::empty())?;

        reader.for_each_entries(|entry, _| {
            if !entry.is_directory() {
                entries.push(PathBuf::from(entry.name()));
            }
            Ok(true)
        })?;

        Ok(Self {
            path: path.into(),
            entries,
        })
    }
}

impl Package for SevenZArchiveHandler {
    fn extract(&mut self, output_dir: &Path) -> Result<PathBuf> {
        // sevenz_rust joins entry names onto the destination unsanitized, so
        // recompute each path ourselves and reject escaping entries
        let dest = output_dir.to_path_buf();
        sevenz_rust::decompress_file_with_extract_fn(&self.path, output_dir, |entry, reader, _| {
            let safe_path = sanitized_join(&dest, entry.name())
                .ok_or_else(|| sevenz_rust::Error::other(format!("Unsafe path in archive: {}", entry.name())))?;
            sevenz_rust::default_entry_extract_fn(entry, reader, &safe_path)
        })?;
        Ok(output_dir.into())
    }

    #[cfg(windows)]
    fn is_executable(&self, path: &Path) -> bool {
        crate::package::has_exe_extension(path)
    }

    #[cfg(unix)]
    fn is_executable(&self, path: &Path) -> bool {
        // sevenz-rust doesn't expose unix mode bits, so use a name-based heuristic:
        // files with no extension under a path containing "bin" are likely executables
        let has_no_extension = path.extension().is_none();
        let under_bin = path.components().any(|c| c.as_os_str() == "bin");
        has_no_extension && under_bin
    }

    fn list(&self) -> Vec<PathBuf> {
        self.entries.clone()
    }

    fn source(&self) -> &Path {
        &self.path
    }
}
