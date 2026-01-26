#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod core;
mod package;
mod platform;
mod state;
mod ui;

use crate::package::open_package;
use crate::state::config::{APP_TITLE, default_options};
use crate::ui::dialogs::{show_error_message, show_warning_message};
use crate::ui::self_install_app::SelfInstallApp;

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
    let args = std::env::args().nth(1);
    match args {
        None => self_install(),
        Some(arg) => zip_install(arg),
    }
}
