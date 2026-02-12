#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod core;
mod package;
mod platform;
mod state;
mod ui;

use crate::app::App;
use crate::package::open_package;
use crate::state::config::Config;
use crate::state::persistable::Persistable;
use crate::ui::dialogs::show_error_message;
use crate::ui::zip_install::ZipInstallView;

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

            App::new(Box::new(ZipInstallView::new(package, config)))
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
