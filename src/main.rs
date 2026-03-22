#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod core;
mod package;
mod platform;
mod state;
mod ui;

use crate::app::App;
use crate::app::routing::Route;
use crate::app::state::{AppData, SharedState};
use crate::package::open_package;
use crate::state::config::Config;
use crate::state::index::InstallIndex;
use crate::state::persistable::Persistable;
use crate::ui::View;
use crate::ui::dialogs::{show_error_message, show_warning_message};
use crate::ui::install_view::InstallView;
use crate::ui::setup_view::SetupView;
use crate::ui::update_view::UpdateView;

use eframe::NativeOptions;

fn main() -> eframe::Result<()> {
    let config = Config::load().unwrap_or_default();

    let (data, route, view): (AppData, Route, Box<dyn View>) = match std::env::args().nth(1) {
        None => {
            let data = AppData {
                package: None,
                shared: SharedState::from_config(&config),
                candidates: vec![],
            };

            let view = Box::new(SetupView::new(config.is_installed()));
            (data, Route::Setup, view)
        }

        Some(arg) => {
            let archive_path = std::path::Path::new(&arg);

            let package = match open_package(archive_path) {
                Ok(pkg) => pkg,
                Err(err) => {
                    show_error_message(&format!("Failed to open archive: {}", err));
                    return Ok(());
                }
            };
            let candidates = package.candidates();

            if candidates.is_empty() {
                show_warning_message("No valid candidates found in the archive.");
                return Ok(());
            }

            let index = InstallIndex::load().unwrap_or_default();
            let has_installed_apps = !index.entries.is_empty();

            let detected_target = core::detect_update(package.as_ref(), &config);

            let data = AppData {
                package: Some(package),
                shared: SharedState::from_config(&config),
                candidates,
            };

            let (route, view): (Route, Box<dyn View>) = match detected_target {
                Some(target) => (Route::Update(target.clone()), Box::new(UpdateView::new(target))),
                None => (Route::Install, Box::new(InstallView::new(has_installed_apps))),
            };

            (data, route, view)
        }
    };

    let app = App::new(data, route, view);

    let mut viewport = app.viewport();
    if let Ok(icon) = eframe::icon_data::from_png_bytes(include_bytes!("../assets/icon.png")) {
        viewport = viewport.with_icon(std::sync::Arc::new(icon));
    }

    eframe::run_native(
        "zip-install",
        NativeOptions {
            viewport,
            ..Default::default()
        },
        Box::new(|_| Ok(Box::new(app))),
    )
}
