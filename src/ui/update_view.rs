use crate::app::routing::{Route, ViewAction, ViewContext};
use crate::state::index::InstalledApp;
use crate::ui::View;
use crate::ui::constants::*;

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

    fn ui(&mut self, ui: &mut Ui, ctx: &mut ViewContext, action: &mut dyn FnMut(ViewAction)) {
        let outer_width = ui.available_width();

        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.add_space(PADDING_TOP);
            ui.set_max_width(outer_width * PADDING_RATIO);

            ui.with_layout(Layout::top_down(Align::Min), |ui| {
                let width = ui.available_width();

                ui.label(RichText::new("Update detected for:"));
                ui.label(RichText::new(&self.target.app_name).strong());

                ui.add_space(SECTION_SPACING);

                ui.checkbox(&mut ctx.shared.checkbox_shortcut_desktop, "Create Desktop shortcut");
                ui.checkbox(&mut ctx.shared.checkbox_shortcut_menu, "Add to Start Menu");
                ui.checkbox(&mut ctx.shared.checkbox_remove_package, "Remove after install");

                ui.add_space(SECTION_SPACING);

                if ui.add_sized([width, BTN_MAIN_HEIGHT], Button::new("Update")).clicked() {
                    // TODO: implement update logic (extract to existing directory)
                    action(ViewAction::Close);
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
