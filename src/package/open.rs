use std::path::Path;

use anyhow::Result;

use super::{
    GzipStandalone, Package, SevenZArchiveHandler, StandaloneExecutable, TarGzArchiveHandler, ZipArchiveHandler,
};

enum PackageKind {
    TarGz,
    Gzip,
    Zip,
    SevenZ,
    Standalone,
}

// Ordered by suffix length so compound extensions (e.g. `.tar.gz`) win over `.gz`.
const PACKAGE_HANDLERS: &[(&str, PackageKind)] = &[
    (".tar.gz", PackageKind::TarGz),
    (".tgz", PackageKind::TarGz),
    (".tar", PackageKind::TarGz),
    (".gz", PackageKind::Gzip),
    (".zip", PackageKind::Zip),
    (".7z", PackageKind::SevenZ),
    (".exe", PackageKind::Standalone),
    (".bat", PackageKind::Standalone),
];

pub fn open_package(path: &Path) -> Result<Box<dyn Package>> {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    let kind = PACKAGE_HANDLERS
        .iter()
        .find(|(suffix, _)| name.ends_with(suffix))
        .map(|(_, kind)| kind)
        .ok_or_else(|| anyhow::anyhow!("Unsupported file format: {}", name))?;

    Ok(match kind {
        PackageKind::TarGz => Box::new(TarGzArchiveHandler::open(path)?),
        PackageKind::Gzip => Box::new(GzipStandalone::open(path)?),
        PackageKind::Zip => Box::new(ZipArchiveHandler::open(path)?),
        PackageKind::SevenZ => Box::new(SevenZArchiveHandler::open(path)?),
        PackageKind::Standalone => Box::new(StandaloneExecutable::open(path)?),
    })
}
