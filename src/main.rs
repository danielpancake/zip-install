#![windows_subsystem = "windows"]

use crate::{
    archive::open_archive,
    config::{APP_TITLE, default_options},
    messages::{show_error_message, show_warning_message},
};

mod app;
mod archive;
mod config;
mod installer;
mod messages;
mod models;
mod shortcuts;

fn self_install() {}

fn zip_install(arg: String) {
    let archive_path = std::path::Path::new(&arg);

    let archive = match open_archive(archive_path) {
        Ok(archive) => archive,
        Err(err) => {
            show_error_message(&format!("Failed to open archive: {}", err));
            return;
        }
    };

    let app = app::App::new(archive);

    if app.executables.is_empty() {
        show_warning_message("No executable files were found in the archive.");
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
