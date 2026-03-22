// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod core;
mod package;
mod platform;
mod state;
mod ui;

use crate::app::App;
use crate::app::routing::SharedState;
use crate::core::fingerprint::Fingerprint;
use crate::package::open_package;
use crate::state::config::Config;
use crate::state::index::{AppMatcher, InstallIndex};
use crate::state::paths;
use crate::state::persistable::Persistable;
use crate::ui::View;
use crate::ui::dialogs::show_error_message;
use crate::ui::install_view::InstallView;
use crate::ui::update_view::UpdateView;

use eframe::NativeOptions;

fn main() -> eframe::Result<()> {
    let config = Config::load().unwrap_or_default();

    let app = match std::env::args().nth(1) {
        None => todo!(),

        Some(arg) => {
            let archive_path = std::path::Path::new(&arg);

            let package = match open_package(archive_path) {
                Ok(pkg) => pkg,
                Err(err) => {
                    show_error_message(&format!("Failed to open archive: {}", err));
                    return Ok(());
                }
            };

            let shared = SharedState::from_config(&config);

            let index = InstallIndex::load().unwrap_or_default();
            let has_installed_apps = !index.entries.is_empty();

            let view: Box<dyn View> = match detect_update(package.as_ref(), &config) {
                Some(target) => Box::new(UpdateView::new(target)),
                None => Box::new(InstallView::new(has_installed_apps)),
            };

            App::new(package, shared, view)
        }
    };

    eframe::run_native(
        "zip-install",
        NativeOptions {
            viewport: app.viewport(),
            ..Default::default()
        },
        Box::new(|_| Ok(Box::new(app))),
    )
}

fn detect_update(package: &dyn crate::package::Package, config: &Config) -> Option<crate::state::index::InstalledApp> {
    let fingerprint = Fingerprint::from_package(package).ok()?;
    eprintln!("[debug] archive fingerprint: {:?}", fingerprint);

    let packages_dir = paths::packages_dir().ok()?;
    let mut matcher = AppMatcher::new();
    matcher.scan_installations(packages_dir.as_path()).ok()?;

    for (path, fp) in &matcher.known_apps {
        let score = fp.similarity(&fingerprint);
        eprintln!("[debug] installed {:?} score={:.4} fingerprint={:?}", path, score, fp);
    }

    let (path, score) = matcher.find_match(&fingerprint, config.match_threshold)?;
    eprintln!("[debug] best match: {:?} score={:.4}", path, score);

    let uuid = path.file_name()?.to_string_lossy().into_owned();
    let index = InstallIndex::load().unwrap_or_default();

    index.entries.get(&uuid).cloned()
}
