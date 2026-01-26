use eframe::egui;

use crate::core::installer::install;
use crate::core::models::ApplicationEntry;
use crate::package::Package;
use crate::state::config::MIN_WINDOW_WIDTH;
use crate::ui::dialogs::{show_error_message, show_info_message};

pub struct App {
    pub archive: Box<dyn Package>,

    pub executables: Vec<ApplicationEntry>,
    pub selected_index: usize,

    pub create_desktop: bool,
    pub create_start_menu: bool,
}

impl App {
    pub fn new(mut archive: Box<dyn Package>) -> Self {
        let executables = archive.candidates();

        Self {
            archive,
            executables,
            selected_index: 0,
            create_desktop: true,
            create_start_menu: true,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let panel_width = MIN_WINDOW_WIDTH.min(ui.available_width() * 0.9);

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(12.0);

                ui.allocate_ui(egui::vec2(panel_width, ui.available_height()), |ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                        ui.label(egui::RichText::new("Select executable to install").small());

                        ui.add_space(6.0);

                        if self.executables.is_empty() {
                            ui.label("No executables found.");
                            return;
                        }

                        let width = ui.available_width();
                        let has_multiple_exes = self.executables.len() > 1;

                        ui.add_enabled_ui(has_multiple_exes, |ui| {
                            egui::ComboBox::from_id_salt("exe_select")
                                .width(width)
                                .selected_text(&self.executables[self.selected_index].name)
                                .show_ui(ui, |ui| {
                                    for (i, exe) in self.executables.iter().enumerate() {
                                        ui.selectable_value(&mut self.selected_index, i, &exe.name);
                                    }
                                });
                        });

                        ui.add_space(8.0);

                        ui.checkbox(&mut self.create_desktop, "Desktop shortcut");
                        ui.checkbox(&mut self.create_start_menu, "Start Menu shortcut");

                        ui.add_space(14.0);

                        let button_width = (ui.available_width() - 8.0) / 2.0;

                        ui.horizontal(|ui| {
                            ui.add_sized([button_width, 26.0], egui::Button::new("Install"))
                                .clicked()
                                .then(|| {
                                    match install(
                                        self.archive.as_mut(),
                                        self.executables[self.selected_index].clone(),
                                        self.create_desktop,
                                        self.create_start_menu,
                                    ) {
                                        Ok(_) => {
                                            show_info_message("Application installed successfully.")
                                        }
                                        Err(e) => {
                                            show_error_message(&format!(
                                                "Failed to install! {}",
                                                e
                                            ));
                                        }
                                    }
                                    ctx.send_viewport_cmd(egui::ViewportCommand::Close)
                                });

                            ui.add_sized([button_width, 26.0], egui::Button::new("Cancel"))
                                .clicked()
                                .then(|| ctx.send_viewport_cmd(egui::ViewportCommand::Close));
                        });
                    });
                });
            });
        });
    }
}
