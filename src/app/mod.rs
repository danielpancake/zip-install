pub mod routing;

use crate::app::routing::{Route, ViewAction};
use crate::ui::View;
use crate::ui::viewport::apply_viewport_builder;
use crate::ui::zip_install::ZipInstallView;

use eframe::egui;

pub struct App {
    view: Box<dyn View>,
}

impl App {
    pub fn new(view: Box<dyn View>) -> App {
        Self { view }
    }

    pub fn handle_action(&mut self, ctx: &egui::Context, action: ViewAction) {
        match action {
            ViewAction::Navigate(route) => {
                self.view = match route {
                    Route::ZipInstall(package) => Box::new(ZipInstallView::new(package)),
                    _ => todo!(),
                };
                apply_viewport_builder(ctx, self.view.viewport());
            }
            ViewAction::Close => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
        }
    }

    pub fn viewport(&self) -> egui::ViewportBuilder {
        self.view.viewport()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let action = {
            let mut captured = None;

            egui::CentralPanel::default().show(ctx, |ui| {
                self.view.ui(ui, &mut |a| captured = Some(a));
            });

            captured
        };

        if let Some(act) = action {
            self.handle_action(ctx, act);
        }
    }
}
