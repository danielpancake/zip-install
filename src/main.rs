#![windows_subsystem = "windows"]

use crate::{
    package::open_package,
    state::config::{default_options, APP_TITLE},
    ui::messages::{show_error_message, show_warning_message},
    ui::self_install_app::SelfInstallApp,
};

mod app;
mod core;
mod package;
mod platform;
mod state;
mod ui;

fn self_install() {
    let app = SelfInstallApp::new();

    eframe::run_native(
        APP_TITLE,
        default_options(),
        Box::new(|_cc| Ok(Box::new(app))),
    )
    .unwrap();
}

fn zip_install(arg: String) {
    let archive_path = std::path::Path::new(&arg);

    let archive = match open_package(archive_path) {
        Ok(archive) => archive,
        Err(err) => {
            show_error_message(&format!("Failed to open package: {}", err));
            return;
        }
    };

    let app = app::App::new(archive);

    if app.executables.is_empty() {
        show_warning_message("No executable files were found in the package.");
        return;
    }

    eframe::run_native(
        APP_TITLE,
        default_options(),
        Box::new(|_cc| Ok(Box::new(app))),
    )
    .unwrap();
}

fn main() {
    match std::env::args().nth(1) {
        None => self_install(),
        Some(arg) => zip_install(arg),
    }
}
