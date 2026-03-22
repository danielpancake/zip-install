use crate::app::routing::{ViewAction, ViewContext};
use crate::state::index::InstalledApp;
use crate::ui::View;
use crate::ui::constants::*;
use crate::ui::dialogs::show_confirm_dialog;

use eframe::egui::{Align, Button, ComboBox, Layout, RichText, Ui, ViewportBuilder};

pub struct ManualUpdateView {
    all_packages: Vec<InstalledApp>,
    update_target_index: Option<usize>,
}

impl ManualUpdateView {
    pub fn new(all_packages: Vec<InstalledApp>) -> Self {
        Self {
            all_packages,
            update_target_index: None,
        }
    }
}

impl View for ManualUpdateView {
    fn viewport(&self) -> ViewportBuilder {
        ViewportBuilder::default()
            .with_title("zip-install — Manual Update")
            .with_inner_size([WINDOW_WIDTH, 300.0])
            .with_resizable(false)
            .with_maximize_button(false)
            .with_minimize_button(false)
    }

    fn ui(&mut self, ui: &mut Ui, ctx: &mut ViewContext, action: &mut dyn FnMut(ViewAction)) {
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
                    .selected_text(&ctx.candidates[ctx.shared.candidates_index].display_name)
                    .show_ui(ui, |ui| {
                        for (i, exe) in ctx.candidates.iter().enumerate() {
                            ui.selectable_value(&mut ctx.shared.candidates_index, i, &exe.display_name);
                        }
                    });

                ui.add_space(SECTION_SPACING);

                ui.label(RichText::new("Select installation to update"));
                ui.add_space(LABEL_SPACING);

                let selected_text = self
                    .update_target_index
                    .and_then(|i| self.all_packages.get(i))
                    .map(|e| e.app_name.as_str())
                    .unwrap_or("");

                ComboBox::from_id_salt("update_target")
                    .width(width)
                    .selected_text(selected_text)
                    .show_ui(ui, |ui| {
                        for (i, entry) in self.all_packages.iter().enumerate() {
                            let mut current = self.update_target_index.unwrap_or(usize::MAX);
                            if ui.selectable_value(&mut current, i, &entry.app_name).changed() {
                                self.update_target_index = Some(current);
                            }
                        }
                    });

                ui.add_space(SECTION_SPACING);

                ui.checkbox(&mut ctx.shared.checkbox_shortcut_desktop, "Create Desktop shortcut");
                ui.checkbox(&mut ctx.shared.checkbox_shortcut_menu, "Add to Start Menu");
                ui.checkbox(&mut ctx.shared.checkbox_remove_package, "Remove after install");

                ui.add_space(SECTION_SPACING);

                if ui.add_sized([width, BTN_HEIGHT], Button::new("Cancel")).clicked() {
                    action(ViewAction::Back);
                }

                ui.add_space(SECTION_SPACING);

                let has_target = self.update_target_index.is_some();

                ui.add_enabled_ui(has_target, |ui| {
                    if ui
                        .add_sized([width, BTN_MAIN_HEIGHT], Button::new("Confirm Update"))
                        .clicked()
                    {
                        let target_name = &self.all_packages[self.update_target_index.unwrap()].app_name;
                        if show_confirm_dialog(&format!(
                            "This will overwrite the existing installation of \"{}\".\n\nContinue?",
                            target_name
                        )) {
                            // TODO: implement update logic (extract to selected app's directory)
                            action(ViewAction::Close);
                        }
                    }
                });
            });
        });
    }
}
