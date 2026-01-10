use crate::{config::default_options, utils::show_error_message};

mod app;
mod config;
mod models;
mod utils;

fn main() {
    let executables = vec![
        models::ApplicationEntry {
            name: "MyApp.exe".to_string(),
            path: "MyApp/MyApp.exe".to_string(),
        },
        models::ApplicationEntry {
            name: "MyAppLauncher.exe".to_string(),
            path: "MyApp/Launcher.exe".to_string(),
        },
        models::ApplicationEntry {
            name: "Uninstall.exe".to_string(),
            path: "MyApp/Uninstall.exe".to_string(),
        },
    ];

    if executables.is_empty() {
        show_error_message("No executable files were found in the archive.");
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
