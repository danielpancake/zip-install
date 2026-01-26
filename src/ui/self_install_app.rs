use crate::{
    state::config::MIN_WINDOW_WIDTH,
    state::install_state::InstallState,
    ui::messages::{show_error_message, show_info_message},
};
use eframe::egui;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Screen {
    Install,
    AppList,
}

pub struct SelfInstallApp {
    install_state: InstallState,
    current_screen: Screen,
}

impl SelfInstallApp {
    pub fn new() -> Self {
        let install_state = InstallState::load().unwrap_or_default();

        Self {
            install_state,
            current_screen: Screen::Install,
        }
    }

    fn handle_install(&mut self) {
        match self.install_state.mark_installed() {
            Ok(_) => show_info_message("Application installed successfully."),
            Err(e) => show_error_message(&format!("Failed to install: {}", e)),
        }
    }

    fn handle_uninstall(&mut self) {
        match self.install_state.mark_uninstalled() {
            Ok(_) => show_info_message("Application uninstalled successfully."),
            Err(e) => show_error_message(&format!("Failed to uninstall: {}", e)),
        }
    }

    fn handle_repair(&mut self) {
        show_info_message("Repair functionality not yet implemented.");
    }

    fn render_install_screen(&mut self, ui: &mut egui::Ui) {
        let panel_width = MIN_WINDOW_WIDTH.min(ui.available_width() * 0.9);

        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.add_space(12.0);
            ui.set_max_width(panel_width);

            ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                ui.label(
                    egui::RichText::new("Archive Installation Utility")
                        .size(16.0)
                        .strong(),
                );

                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new(
                        "Install executables from ZIP archives and standalone EXE files.",
                    )
                    .small()
                    .color(ui.visuals().weak_text_color()),
                );

                ui.add_space(16.0);

                let status_text = if self.install_state.is_installed() {
                    "Status: Installed"
                } else {
                    "Status: Not Installed"
                };
                ui.label(egui::RichText::new(status_text).small());

                ui.add_space(12.0);

                let button_width = ui.available_width();

                if self.install_state.is_installed() {
                    if ui
                        .add_sized([button_width, 28.0], egui::Button::new("Repair"))
                        .clicked()
                    {
                        self.handle_repair();
                    }

                    ui.add_space(6.0);

                    if ui
                        .add_sized([button_width, 28.0], egui::Button::new("Uninstall"))
                        .clicked()
                    {
                        self.handle_uninstall();
                    }
                } else if ui
                    .add_sized([button_width, 28.0], egui::Button::new("Install"))
                    .clicked()
                {
                    self.handle_install();
                }

                ui.add_space(6.0);

                if ui
                    .add_sized([button_width, 28.0], egui::Button::new("App List"))
                    .clicked()
                {
                    self.current_screen = Screen::AppList;
                }

                ui.add_space(12.0);
            });
        });
    }
}

impl eframe::App for SelfInstallApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| self.render_install_screen(ui));
    }
}
