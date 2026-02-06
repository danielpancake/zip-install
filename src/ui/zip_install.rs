use crate::core::models::ApplicationEntry;
use crate::package::Package;
use crate::ui::View;
use crate::ui::dialogs::{show_error_message, show_info_message};
use crate::{app::routing::ViewAction, core::installer::install};

use eframe::egui::{Button, ComboBox, RichText, Ui, ViewportBuilder};

pub struct ZipInstallView {
    pub package: Box<dyn Package>,

    candidates: Vec<ApplicationEntry>,
    candidates_index: usize,

    checkbox_shortcut_desktop: bool,
    checkbox_shortcut_menu: bool,
    checkbox_remove_package: bool,
}

impl ZipInstallView {
    pub fn new(package: Box<dyn Package>) -> Self {
        let candidates = package.candidates();

        Self {
            package,
            candidates,

            candidates_index: 0,

            // TODO: move defaults to app config
            checkbox_shortcut_desktop: true,
            checkbox_shortcut_menu: true,
            checkbox_remove_package: false,
        }
    }
}

impl View for ZipInstallView {
    fn viewport(&self) -> ViewportBuilder {
        ViewportBuilder::default()
            .with_inner_size([240.0, 320.0])
            .with_resizable(false)
            .with_maximize_button(false)
            .with_minimize_button(false)
    }

    fn ui(&mut self, ui: &mut Ui, action: &mut dyn FnMut(ViewAction)) {
        let width = ui.available_width();

        let btn_separator = 8.0;
        let btn_width = (ui.available_width() - btn_separator) / 2.0;
        let btn_height = 26.0;

        ui.label(RichText::new("Select executable to install"));

        ComboBox::from_id_salt("install")
            .width(width)
            .selected_text(&self.candidates[self.candidates_index].name)
            .show_ui(ui, |ui| {
                for (i, exe) in self.candidates.iter().enumerate() {
                    ui.selectable_value(&mut self.candidates_index, i, &exe.name);
                }
            });

        ui.checkbox(&mut self.checkbox_shortcut_desktop, "Create Desktop shortcut");
        ui.checkbox(&mut self.checkbox_shortcut_menu, "Add to Start Menu");
        ui.checkbox(&mut self.checkbox_remove_package, "Remove after install");

        ui.horizontal(|ui| {
            if ui.add_sized([btn_width, btn_height], Button::new("Install")).clicked() {
                match install(
                    self.package.as_mut(),
                    self.candidates[self.candidates_index].clone(),
                    self.checkbox_shortcut_desktop,
                    self.checkbox_shortcut_menu,
                ) {
                    Ok(_) => {
                        show_info_message("Application installed successfully.");
                        action(ViewAction::Close);
                    }
                    Err(e) => {
                        show_error_message(&format!("Failed to install! {}", e));
                    }
                }
                action(ViewAction::Close);
            }

            if ui.add_sized([btn_width, btn_height], Button::new("Cancel")).clicked() {
                action(ViewAction::Close);
            }
        });

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(8.0);

        ui.label(RichText::new("Or update an existing installation"));
        ComboBox::from_id_salt("update_existing")
            .width(ui.available_width())
            .show_ui(ui, |_| {});

        if ui
            .add_sized([width, btn_height], Button::new("Confirm Update"))
            .clicked()
        {
            todo!()
        }
    }
}
