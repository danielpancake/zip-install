use std::path::PathBuf;

#[derive(Clone)]
pub struct ApplicationEntry {
    pub name: String,
    pub path: PathBuf,
}

impl From<PathBuf> for ApplicationEntry {
    fn from(path: PathBuf) -> Self {
        let name = path
            .file_name()
            .map(|name| name.to_string_lossy().into())
            .unwrap_or_default();

        Self { name, path }
    }
}
