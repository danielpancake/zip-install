use crate::app::routing::{Route, SharedState, ViewAction};
use crate::package::{Candidate, Package};
use crate::state::index::{InstallIndex, InstalledApp};
use crate::state::persistable::Persistable;
use crate::ui::View;
use crate::ui::constants::*;

use eframe::egui::{Align, Button, ComboBox, Layout, RichText, Ui, ViewportBuilder};

pub struct ManualUpdateView {
    package: Option<Box<dyn Package>>,

    candidates: Vec<Candidate>,
    shared: SharedState,

    all_packages: Vec<InstalledApp>,
    update_target_index: usize,
}

impl ManualUpdateView {
    pub fn new(package: Box<dyn Package>, shared: SharedState) -> Self {
        let candidates = package.candidates();
        let index = InstallIndex::load().unwrap_or_default();
        let all_packages: Vec<InstalledApp> = index.entries.values().cloned().collect();

        Self {
            candidates,
            shared,
            all_packages,
            update_target_index: 0,
            package: Some(package),
        }
    }
}

impl View for ManualUpdateView {
    fn viewport(&self) -> ViewportBuilder {
        ViewportBuilder::default()
            .with_inner_size([WINDOW_WIDTH, 300.0])
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

                ui.label(RichText::new("Select executable"));
                ui.add_space(LABEL_SPACING);

                ComboBox::from_id_salt("manual_update_exe")
                    .width(width)
                    .selected_text(&self.candidates[self.shared.candidates_index].file_name)
                    .show_ui(ui, |ui| {
                        for (i, exe) in self.candidates.iter().enumerate() {
                            ui.selectable_value(&mut self.shared.candidates_index, i, &exe.file_name);
                        }
                    });

                ui.add_space(SECTION_SPACING);

                ui.label(RichText::new("Select installation to update"));
                ui.add_space(LABEL_SPACING);

                ComboBox::from_id_salt("update_target")
                    .width(width)
                    .selected_text(
                        self.all_packages
                            .get(self.update_target_index)
                            .map(|e| e.base_name.as_str())
                            .unwrap_or("No installations found"),
                    )
                    .show_ui(ui, |ui| {
                        for (i, entry) in self.all_packages.iter().enumerate() {
                            ui.selectable_value(&mut self.update_target_index, i, &entry.base_name);
                        }
                    });

                ui.add_space(SECTION_SPACING);

                ui.checkbox(&mut self.shared.checkbox_shortcut_desktop, "Create Desktop shortcut");
                ui.checkbox(&mut self.shared.checkbox_shortcut_menu, "Add to Start Menu");
                ui.checkbox(&mut self.shared.checkbox_remove_package, "Remove after install");

                ui.add_space(SECTION_SPACING);

                if ui.add_sized([width, BTN_HEIGHT], Button::new("Cancel")).clicked() {
                    if let Some(pkg) = self.package.take() {
                        action(ViewAction::Navigate(Route::Install(pkg, self.shared.take())));
                    }
                }

                ui.add_space(SECTION_SPACING);

                if ui
                    .add_sized([width, BTN_MAIN_HEIGHT], Button::new("Confirm Update"))
                    .clicked()
                {
                    // TODO: implement update logic (extract to selected app's directory)
                }
            });
        });
    }
}
