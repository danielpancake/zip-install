#![windows_subsystem = "windows"]

use crate::{
    archive::open_archive,
    config::default_options,
    utils::{show_error_message, show_warning_message},
};

mod app;
mod archive;
mod config;
mod models;
mod utils;

fn self_install() {}

fn zip_install(arg: String) {
    let archive_path = std::path::Path::new(&arg);

    let mut archive = match open_archive(archive_path) {
        Ok(archive) => archive,
        Err(err) => {
            show_error_message(&format!("Failed to open archive: {}", err));
            return;
        }
    };

    let executables = archive.candidates();

    if executables.is_empty() {
        show_warning_message("No executable files were found in the archive.");
        return;
    }

    let app = app::App::new(executables);

    eframe::run_native(
        "zip-install",
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
