mod candidate;
mod dir;
mod gzip;
mod open;
mod sevenz;
mod standalone;
mod targz;
mod zip;

use std::path::{Path, PathBuf};

use anyhow::Result;

pub use candidate::{Candidate, strip_version};
pub use dir::DirPackage;
pub use gzip::GzipStandalone;
pub use open::open_package;
pub use sevenz::SevenZArchiveHandler;
pub use standalone::StandaloneExecutable;
pub use targz::TarGzArchiveHandler;
pub use zip::ZipArchiveHandler;

use candidate::disambiguate_candidates;

#[cfg(windows)]
pub(crate) fn has_exe_extension(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("exe"))
}

pub trait Package: Send {
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
