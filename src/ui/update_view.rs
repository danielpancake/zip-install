use crate::app::routing::{Route, ViewAction};
use crate::app::state::AppData;
use crate::core::installer::update;
use crate::state::index::{InstallIndex, InstalledApp};
use crate::state::persistable::Persistable;
use crate::ui::View;
use crate::ui::constants::*;
use crate::ui::dialogs::{show_error_message, show_info_message, show_warning_message};

use eframe::egui::{Align, Button, Layout, RichText, Ui, ViewportBuilder};

pub struct UpdateView {
    target: InstalledApp,
}

impl UpdateView {
    pub fn new(target: InstalledApp) -> Self {
        Self { target }
    }
}

impl View for UpdateView {
    fn viewport(&self) -> ViewportBuilder {
        ViewportBuilder::default()
            .with_title("zip-install — Update")
            .with_resizable(false)
            .with_inner_size([WINDOW_WIDTH, 260.0])
            .with_maximize_button(false)
            .with_minimize_button(false)
    }

    fn ui(&mut self, ui: &mut Ui, data: &mut AppData, action: &mut dyn FnMut(ViewAction)) {
        let outer_width = ui.available_width();

        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.add_space(PADDING_TOP);
            ui.set_max_width(outer_width * PADDING_RATIO);

            ui.with_layout(Layout::top_down(Align::Min), |ui| {
                let width = ui.available_width();

                ui.label(RichText::new("Update detected for:"));
                ui.label(RichText::new(&self.target.app_name).strong());

                ui.add_space(SECTION_SPACING);

                ui.checkbox(&mut data.shared.checkbox_shortcut_desktop, "Create Desktop shortcut");
                ui.checkbox(&mut data.shared.checkbox_shortcut_menu, LABEL_APP_LAUNCHER);
                ui.checkbox(&mut data.shared.checkbox_remove_package, "Remove after install");

                ui.add_space(SECTION_SPACING);

                if ui.add_sized([width, BTN_MAIN_HEIGHT], Button::new("Update")).clicked() {
                    if let (Some(package), Some(candidate)) = (
                        data.package.as_mut(),
                        data.candidates.get(data.shared.candidates_index).cloned(),
                    ) {
                        match update(
                            package.as_mut(),
                            candidate.clone(),
                            &self.target.uuid,
                            data.shared.checkbox_shortcut_desktop,
                            data.shared.checkbox_shortcut_menu,
                        ) {
                            Ok(()) => {
                                let mut index = InstallIndex::load().unwrap_or_default();
                                index.add_entry(&self.target.uuid, InstalledApp::from(&candidate));
                                if let Err(e) = index.save() {
                                    show_error_message(&format!("Failed to save index: {}", e));
                                }
                                if data.shared.checkbox_remove_package {
                                    if let Some(pkg) = data.package.as_ref() {
                                        if let Err(e) = std::fs::remove_file(pkg.source()) {
                                            show_warning_message(&format!("Failed to remove package: {}", e));
                                        }
                                    }
                                }
                                show_info_message("Application updated successfully.");
                                action(ViewAction::Close);
                            }
                            Err(e) => {
                                show_error_message(&format!("Failed to update! {}", e));
                            }
                        }
                    }
                }

                ui.add_space(SECTION_SPACING);

                if ui
                    .add_sized([width, BTN_HEIGHT], Button::new("Update another..."))
                    .clicked()
                {
                    action(ViewAction::Navigate(Route::ManualUpdate));
                }

                ui.add_space(SECTION_SPACING);

                if ui
                    .add_sized([width, BTN_HEIGHT], Button::new("Install as new..."))
                    .clicked()
                {
                    action(ViewAction::Navigate(Route::Install));
                }
            });
        });
    }
}
