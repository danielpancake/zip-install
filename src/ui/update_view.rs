use crate::app::job;
use crate::app::routing::{Route, ViewAction};
use crate::app::state::AppData;
use crate::state::index::InstalledApp;
use crate::ui::View;
use crate::ui::constants::*;

use eframe::egui::{Align, Button, ComboBox, Layout, RichText, Ui, ViewportBuilder};

pub struct UpdateView {
    target: InstalledApp,
    exe_preselected: bool,
}

impl UpdateView {
    pub fn new(target: InstalledApp) -> Self {
        Self {
            target,
            exe_preselected: false,
        }
    }
}

impl View for UpdateView {
    fn viewport(&self) -> ViewportBuilder {
        ViewportBuilder::default()
            .with_title("zip-install — Update")
            .with_resizable(false)
            .with_inner_size([WINDOW_WIDTH, 340.0])
            .with_maximize_button(false)
            .with_minimize_button(false)
    }

    fn ui(&mut self, ui: &mut Ui, data: &mut AppData, action: &mut dyn FnMut(ViewAction)) {
        let outer_width = ui.available_width();

        if data.candidates.is_empty() {
            return;
        }

        // Default to the target's recorded executable, not whatever the
        // shared index happens to hold, so an update can't silently switch
        // the app's main exe. Done once per entry into this view
        if !self.exe_preselected {
            self.exe_preselected = true;
            data.shared.candidates_index = self.target.matching_candidate(&data.candidates).unwrap_or(0);
        }

        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.add_space(PADDING_TOP);
            ui.set_max_width(outer_width * PADDING_RATIO);

            ui.with_layout(Layout::top_down(Align::Min), |ui| {
                let width = ui.available_width();

                ui.label(RichText::new("Update detected for:"));
                ui.label(RichText::new(&self.target.app_name).strong());

                ui.add_space(SECTION_SPACING);

                ui.label(RichText::new("Select executable"));
                ui.add_space(LABEL_SPACING);

                ComboBox::from_id_salt("update_exe")
                    .width(width)
                    .selected_text(&data.candidates[data.shared.candidates_index].display_name)
                    .show_ui(ui, |ui| {
                        for (i, exe) in data.candidates.iter().enumerate() {
                            ui.selectable_value(&mut data.shared.candidates_index, i, &exe.display_name);
                        }
                    });

                ui.add_space(SECTION_SPACING);

                ui.checkbox(&mut data.shared.checkbox_shortcut_desktop, "Create Desktop shortcut");
                ui.checkbox(&mut data.shared.checkbox_shortcut_menu, LABEL_APP_LAUNCHER);
                ui.checkbox(&mut data.shared.checkbox_remove_package, "Remove after install");

                ui.add_space(SECTION_SPACING);

                if ui.add_sized([width, BTN_MAIN_HEIGHT], Button::new("Update")).clicked()
                    && let Some(candidate) = data.candidates.get(data.shared.candidates_index).cloned()
                    && let Some(package) = data.package.take()
                {
                    data.job = Some(job::spawn(
                        package,
                        candidate,
                        Some(&self.target),
                        data.shared.checkbox_shortcut_desktop,
                        data.shared.checkbox_shortcut_menu,
                        data.shared.checkbox_remove_package,
                        ui.ctx().clone(),
                    ));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::state::{AppData, SharedState};
    use crate::package::Candidate;
    use crate::state::index::StoredFingerprint;

    /// Runs one headless frame of the view so the first-frame preselection fires.
    fn run_frame(view: &mut UpdateView, data: &mut AppData) {
        let ctx = eframe::egui::Context::default();
        let input = eframe::egui::RawInput {
            screen_rect: Some(eframe::egui::Rect::from_min_size(
                Default::default(),
                eframe::egui::vec2(WINDOW_WIDTH, 1000.0),
            )),
            ..Default::default()
        };

        let _ = ctx.run(input, |ctx| {
            eframe::egui::CentralPanel::default().show(ctx, |ui| {
                view.ui(ui, data, &mut |_| {});
            });
        });
    }

    #[test]
    fn preselects_target_executable() {
        let target = InstalledApp {
            uuid: "uuid-1".into(),
            app_name: "SuperApp".into(),
            file_name: "app.exe".into(),
            main_path: "app/app.exe".into(),
            installed_at: String::new(),
            shortcuts: Vec::new(),
            fingerprint: StoredFingerprint::default(),
        };
        let mut data = AppData {
            package: None,
            shared: SharedState {
                candidates_index: 0,
                checkbox_shortcut_desktop: true,
                checkbox_shortcut_menu: true,
                checkbox_remove_package: false,
            },
            candidates: vec![
                Candidate::from(std::path::PathBuf::from("app/uninstall.exe")),
                Candidate::from(std::path::PathBuf::from("app/app.exe")),
            ],
            job: None,
        };

        run_frame(&mut UpdateView::new(target), &mut data);

        assert_eq!(
            data.shared.candidates_index, 1,
            "must select the installed app's exe, not candidate 0"
        );
    }
}
