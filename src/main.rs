#![windows_subsystem = "windows"]

use eframe::egui;

struct InstallerApp {
    exes: Vec<ExeEntry>,
    selected_index: usize,
    create_desktop: bool,
    create_start_menu: bool,
}

struct ExeEntry {
    display_name: String,
    archive_path: String,
}

impl eframe::App for InstallerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let panel_width = (360.0_f32).min(ui.available_width() * 0.9);

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(12.0);

                ui.allocate_ui(egui::vec2(panel_width, ui.available_height()), |ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                        ui.label(egui::RichText::new("Select executable to install").small());

                        ui.add_space(6.0);

                        if self.exes.is_empty() {
                            ui.label("No executables found.");
                            return;
                        }

                        let width = ui.available_width();

                        egui::ComboBox::from_id_salt("exe_select")
                            .width(width)
                            .selected_text(&self.exes[self.selected_index].display_name)
                            .show_ui(ui, |ui| {
                                for (i, exe) in self.exes.iter().enumerate() {
                                    ui.selectable_value(
                                        &mut self.selected_index,
                                        i,
                                        &exe.display_name,
                                    );
                                }
                            });

                        ui.add_space(8.0);

                        ui.checkbox(&mut self.create_desktop, "Desktop shortcut");
                        ui.checkbox(&mut self.create_start_menu, "Start Menu shortcut");

                        ui.add_space(14.0);

                        let button_width = (ui.available_width() - 8.0) / 2.0;

                        ui.horizontal(|ui| {
                            ui.add_sized([button_width, 26.0], egui::Button::new("Install"));
                            ui.add_sized([button_width, 26.0], egui::Button::new("Cancel"));
                        });

                        ui.add_space(6.0);
                        ui.small("Mock data mode");
                    });
                });
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let exes = vec![
        ExeEntry {
            display_name: "MyApp.exe".to_string(),
            archive_path: "MyApp/MyApp.exe".to_string(),
        },
        ExeEntry {
            display_name: "MyAppLauncher.exe".to_string(),
            archive_path: "MyApp/Launcher.exe".to_string(),
        },
        ExeEntry {
            display_name: "Uninstall.exe".to_string(),
            archive_path: "MyApp/Uninstall.exe".to_string(),
        },
    ];

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([200.0, 200.0])
            .with_resizable(false),
        ..Default::default()
    };

    let app = InstallerApp {
        exes,
        selected_index: 0,
        create_desktop: true,
        create_start_menu: true,
    };

    eframe::run_native(
        "Archive Installer",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    )?;

    Ok(())
}
