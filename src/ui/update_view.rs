use crate::app::routing::{Route, SharedState, ViewAction};
use crate::package::Package;
use crate::state::index::InstalledApp;
use crate::ui::View;
use crate::ui::constants::*;

use eframe::egui::{Align, Button, Layout, RichText, Ui, ViewportBuilder};

pub struct UpdateView {
    package: Option<Box<dyn Package>>,
    target: InstalledApp,

    shared: SharedState,
}

impl UpdateView {
    pub fn new(package: Box<dyn Package>, target: InstalledApp, shared: SharedState) -> Self {
        Self {
            package: Some(package),
            target,
            shared,
        }
    }
}

impl View for UpdateView {
    fn viewport(&self) -> ViewportBuilder {
        ViewportBuilder::default()
            .with_resizable(false)
            .with_inner_size([WINDOW_WIDTH, 320.0])
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

                ui.label(RichText::new("Update detected for:"));
                ui.label(RichText::new(&self.target.base_name).strong());

                ui.add_space(SECTION_SPACING);

                ui.checkbox(&mut self.shared.checkbox_shortcut_desktop, "Create Desktop shortcut");
                ui.checkbox(&mut self.shared.checkbox_shortcut_menu, "Add to Start Menu");
                ui.checkbox(&mut self.shared.checkbox_remove_package, "Remove after install");

                ui.add_space(SECTION_SPACING);

                if ui.add_sized([width, BTN_MAIN_HEIGHT], Button::new("Update")).clicked() {
                    // TODO: implement update logic (extract to existing directory)
                }

                ui.add_space(SECTION_SPACING);

                if ui
                    .add_sized([width, BTN_HEIGHT], Button::new("Install as new"))
                    .clicked()
                {
                    if let Some(pkg) = self.package.take() {
                        action(ViewAction::Navigate(Route::Install(pkg, self.shared.take())));
                    }
                }
            });
        });
    }
}
