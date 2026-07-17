use std::path::PathBuf;

use crate::app::routing::ViewAction;
use crate::app::state::AppData;
use crate::platform::file_explorer::open_file_explorer;
use crate::state::index::{InstallIndex, InstalledApp};
use crate::state::paths;
use crate::state::persistable::Persistable;
use crate::ui::View;
use crate::ui::constants::*;
use crate::ui::dialogs::{show_confirm_dialog, show_error_message, show_warning_message};

use eframe::egui::{Align, Button, Layout, RichText, ScrollArea, Ui, ViewportBuilder};

pub struct AppListView {
    apps: Vec<InstalledApp>,
}

impl AppListView {
    pub fn new(apps: Vec<InstalledApp>) -> Self {
        Self { apps }
    }

    fn install_dir(app: &InstalledApp) -> Option<PathBuf> {
        paths::packages_dir().ok().map(|dir| dir.join(&app.uuid))
    }

    fn launch(app: &InstalledApp) {
        let Some(install_dir) = Self::install_dir(app) else {
            return;
        };
        let exe_path = install_dir.join(&app.main_path);

        let mut command = std::process::Command::new(&exe_path);
        if let Some(parent) = exe_path.parent() {
            command.current_dir(parent);
        }

        if let Err(e) = command.spawn() {
            show_error_message(&format!("Failed to launch \"{}\": {}", app.app_name, e));
        }
    }

    fn open_folder(app: &InstalledApp) {
        let Some(install_dir) = Self::install_dir(app) else {
            return;
        };

        if let Err(e) = open_file_explorer(&install_dir) {
            show_error_message(&format!("Failed to open folder: {}", e));
        }
    }

    /// Deletes the installation, its recorded shortcuts, and the index
    /// entry. Returns true when the app should leave the list
    fn remove(app: &InstalledApp) -> bool {
        if !show_confirm_dialog(&format!(
            "This will permanently delete \"{}\" and its shortcuts.\n\nContinue?",
            app.app_name
        )) {
            return false;
        }

        if let Some(install_dir) = Self::install_dir(app)
            && install_dir.exists()
            && let Err(e) = std::fs::remove_dir_all(&install_dir)
        {
            show_error_message(&format!("Failed to remove installation: {}", e));
            return false;
        }

        for shortcut in &app.shortcuts {
            if let Err(e) = std::fs::remove_file(shortcut)
                && e.kind() != std::io::ErrorKind::NotFound
            {
                show_warning_message(&format!("Failed to remove shortcut {}: {}", shortcut, e));
            }
        }

        let mut index = InstallIndex::load().unwrap_or_default();
        index.remove_entry(&app.uuid);
        if let Err(e) = index.save() {
            show_error_message(&format!("Failed to save index: {}", e));
        }

        true
    }
}

impl View for AppListView {
    fn viewport(&self) -> ViewportBuilder {
        ViewportBuilder::default()
            .with_title("zip-install — Installed Apps")
            .with_inner_size([WINDOW_WIDTH * 1.8, 340.0])
            .with_resizable(false)
            .with_maximize_button(false)
            .with_minimize_button(false)
    }

    fn ui(&mut self, ui: &mut Ui, _data: &mut AppData, action: &mut dyn FnMut(ViewAction)) {
        let outer_width = ui.available_width();

        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.add_space(PADDING_TOP);
            ui.set_max_width(outer_width * PADDING_RATIO);

            ui.with_layout(Layout::top_down(Align::Min), |ui| {
                let width = ui.available_width();

                ui.label(RichText::new("Installed apps").strong());
                ui.add_space(LABEL_SPACING);

                let list_height = ui.available_height() - (BTN_HEIGHT + SECTION_SPACING + PADDING_TOP);

                if self.apps.is_empty() {
                    ui.add_space(SECTION_SPACING);
                    ui.label(RichText::new("No apps installed yet.").weak());
                    ui.add_space(list_height - ui.spacing().interact_size.y);
                } else {
                    let mut removed = None;

                    ScrollArea::vertical()
                        .max_height(list_height)
                        .min_scrolled_height(list_height)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            for (i, app) in self.apps.iter().enumerate() {
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.label(RichText::new(&app.app_name).strong());
                                        ui.label(
                                            RichText::new(format!("installed {}", app.installed_at)).small().weak(),
                                        );
                                    });

                                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                        if ui.add(Button::new("Remove")).clicked() && Self::remove(app) {
                                            removed = Some(i);
                                        }

                                        if ui.add(Button::new("Folder")).clicked() {
                                            Self::open_folder(app);
                                        }

                                        if ui.add(Button::new("Run")).clicked() {
                                            Self::launch(app);
                                        }
                                    });
                                });

                                ui.separator();
                            }
                        });

                    if let Some(i) = removed {
                        self.apps.remove(i);
                    }
                }

                ui.add_space(SECTION_SPACING);

                if ui.add_sized([width, BTN_HEIGHT], Button::new("Back")).clicked() {
                    action(ViewAction::Back);
                }
            });
        });
    }
}
