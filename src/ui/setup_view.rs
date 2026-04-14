use crate::app::routing::ViewAction;
use crate::app::state::AppData;
use crate::core::bootstrap;
use crate::state::paths;
use crate::ui::View;
use crate::ui::constants::*;
use crate::ui::dialogs::{show_error_message, show_info_message};

use eframe::egui::{
    Align, Button, ColorImage, Image, Layout, RichText, TextureHandle, TextureOptions, Ui, Vec2, ViewportBuilder,
};

const ICON_BYTES: &[u8] = include_bytes!("../../assets/icon.png");
const ICON_SIZE: f32 = 64.0;

pub struct SetupView {
    is_installed: bool,
    icon_texture: Option<TextureHandle>,
}

impl SetupView {
    pub fn new(is_installed: bool) -> Self {
        Self {
            is_installed,
            icon_texture: None,
        }
    }

    fn icon(&mut self, ui: &Ui) -> &TextureHandle {
        self.icon_texture.get_or_insert_with(|| {
            let icon_data = eframe::icon_data::from_png_bytes(ICON_BYTES).unwrap();
            let size = [icon_data.width as usize, icon_data.height as usize];
            let color_image = ColorImage::from_rgba_unmultiplied(size, &icon_data.rgba);
            ui.ctx().load_texture("setup-icon", color_image, TextureOptions::LINEAR)
        })
    }
}

impl View for SetupView {
    fn viewport(&self) -> ViewportBuilder {
        ViewportBuilder::default()
            .with_title("zip-install — Setup")
            .with_inner_size([WINDOW_WIDTH * 2.5, 220.0])
            .with_resizable(false)
            .with_maximize_button(false)
            .with_minimize_button(false)
    }

    fn ui(&mut self, ui: &mut Ui, _data: &mut AppData, _action: &mut dyn FnMut(ViewAction)) {
        let outer_width = ui.available_width();

        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.add_space(PADDING_TOP);
            ui.set_max_width(outer_width * PADDING_RATIO);

            ui.with_layout(Layout::top_down(Align::Min), |ui| {
                let width = ui.available_width();

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("zip-install").strong().size(16.0));
                        // ui.label(RichText::new("description...").small());
                    });

                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        let texture = self.icon(ui);
                        ui.add(Image::from_texture(texture).fit_to_exact_size(Vec2::splat(ICON_SIZE)));
                    });
                });

                ui.add_space(LABEL_SPACING);

                if self.is_installed {
                    if ui
                        .add_sized([width, BTN_MAIN_HEIGHT], Button::new("Reinstall"))
                        .clicked()
                    {
                        match bootstrap::reinstall() {
                            Ok(()) => {
                                show_info_message("zip-install has been reinstalled.");
                            }
                            Err(e) => {
                                show_error_message(&format!("Reinstall failed: {}", e));
                            }
                        }
                    }

                    ui.add_space(SECTION_SPACING);

                    if ui
                        .add_sized([width, BTN_MAIN_HEIGHT], Button::new("Uninstall"))
                        .clicked()
                    {
                        match bootstrap::uninstall() {
                            Ok(()) => {
                                self.is_installed = false;
                                show_info_message("zip-install has been uninstalled.");
                            }
                            Err(e) => {
                                show_error_message(&format!("Uninstall failed: {}", e));
                            }
                        }
                    }
                } else {
                    ui.label("This will:");
                    ui.label(format!(
                        "  * Copy zip-install to {}",
                        paths::packages_dir()
                            .unwrap_or_else(|_| "the packages directory".into())
                            .display()
                    ));
                    ui.label(format!("  * Register for {}", bootstrap::EXTENSIONS.join(", ")));

                    ui.add_space(SECTION_SPACING);

                    if ui.add_sized([width, BTN_MAIN_HEIGHT], Button::new("Install")).clicked() {
                        match bootstrap::setup() {
                            Ok(()) => {
                                self.is_installed = true;
                                show_info_message("zip-install has been installed successfully.");
                            }
                            Err(e) => {
                                show_error_message(&format!("Installation failed: {}", e));
                            }
                        }
                    }
                }
            });
        });
    }
}
