use crate::app::routing::{Route, SharedState, ViewAction};
use crate::core::installer::install;
use crate::package::{Candidate, Package};
use crate::state::index::{InstallIndex, InstalledApp};
use crate::state::persistable::Persistable;
use crate::ui::View;
use crate::ui::constants::*;
use crate::ui::dialogs::{show_error_message, show_info_message};

use eframe::egui::{Align, Button, ComboBox, Layout, RichText, Ui, ViewportBuilder};

pub struct InstallView {
    package: Option<Box<dyn Package>>,
    candidates: Vec<Candidate>,

    shared: SharedState,

    has_installed_apps: bool,
}

impl InstallView {
    pub fn new(package: Box<dyn Package>, shared: SharedState) -> Self {
        let index = InstallIndex::load().unwrap_or_default();
        let has_installed_apps = !index.entries.is_empty();

        Self {
            candidates: package.candidates(),
            package: Some(package),
            shared,
            has_installed_apps,
        }
    }
}

impl View for InstallView {
    fn viewport(&self) -> ViewportBuilder {
        ViewportBuilder::default()
            .with_inner_size([WINDOW_WIDTH, 260.0])
            .with_resizable(false)
            .with_maximize_button(false)
            .with_minimize_button(false)
    }

    fn ui(&mut self, ui: &mut Ui, action: &mut dyn FnMut(ViewAction)) {
        let outer_width = ui.available_width();

        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.add_space(PADDING_TOP);
            ui.set_max_width(outer_width * PADDING_RATIO);

            ui.with_layout(Layout::top_down(Align::Min), |ui| {
                let width = ui.available_width();

                ui.label(RichText::new("Select executable to install"));
                ui.add_space(LABEL_SPACING);

                ComboBox::from_id_salt("install_exe")
                    .width(width)
                    .selected_text(&self.candidates[self.shared.candidates_index].file_name)
                    .show_ui(ui, |ui| {
                        for (i, exe) in self.candidates.iter().enumerate() {
                            ui.selectable_value(&mut self.shared.candidates_index, i, &exe.file_name);
                        }
                    });

                ui.add_space(SECTION_SPACING);

                ui.checkbox(&mut self.shared.checkbox_shortcut_desktop, "Create Desktop shortcut");
                ui.checkbox(&mut self.shared.checkbox_shortcut_menu, "Add to Start Menu");
                ui.checkbox(&mut self.shared.checkbox_remove_package, "Remove after install");

                ui.add_space(SECTION_SPACING);

                if ui.add_sized([width, BTN_MAIN_HEIGHT], Button::new("Install")).clicked() {
                    if let Some(package) = self.package.as_mut() {
                        match install(
                            package.as_mut(),
                            self.candidates[self.shared.candidates_index].clone(),
                            self.shared.checkbox_shortcut_desktop,
                            self.shared.checkbox_shortcut_menu,
                        ) {
                            Ok(uuid) => {
                                let mut index = InstallIndex::load().unwrap_or_default();
                                index.add_entry(
                                    &uuid,
                                    InstalledApp::from(&self.candidates[self.shared.candidates_index]),
                                );
                                index.save().unwrap();
                                show_info_message("Application installed successfully.");
                                action(ViewAction::Close);
                            }
                            Err(e) => {
                                show_error_message(&format!("Failed to install! {}", e));
                            }
                        }
                    }
                }

                if self.has_installed_apps {
                    ui.add_space(SECTION_SPACING);

                    if ui
                        .add_sized([width, BTN_HEIGHT], Button::new("Update installed app..."))
                        .clicked()
                    {
                        if let Some(pkg) = self.package.take() {
                            action(ViewAction::Navigate(Route::ManualUpdate(pkg, self.shared.take())));
                        }
                    }
                }
            });
        });
    }
}
