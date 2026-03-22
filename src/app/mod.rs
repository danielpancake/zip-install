pub mod routing;

use crate::app::routing::{Route, SharedState, ViewAction, ViewContext};
use crate::package::{Candidate, Package};
use crate::state::index::{InstallIndex, InstalledApp};
use crate::state::persistable::Persistable;
use crate::ui::View;
use crate::ui::install_view::InstallView;
use crate::ui::manual_update_view::ManualUpdateView;
use crate::ui::update_view::UpdateView;
use crate::ui::viewport::apply_viewport_builder;

use eframe::egui;

pub struct App {
    package: Box<dyn Package>,
    shared: SharedState,
    candidates: Vec<Candidate>,
    view: Box<dyn View>,
    history: Vec<Box<dyn View>>,
}

impl App {
    pub fn new(package: Box<dyn Package>, shared: SharedState, view: Box<dyn View>) -> App {
        let candidates = package.candidates();
        Self {
            package,
            shared,
            candidates,
            view,
            history: Vec::new(),
        }
    }

    fn create_view(&self, route: Route) -> Box<dyn View> {
        match route {
            Route::Install => {
                let index = InstallIndex::load().unwrap_or_default();
                Box::new(InstallView::new(!index.entries.is_empty()))
            }
            Route::Update(target) => Box::new(UpdateView::new(target)),
            Route::ManualUpdate => {
                let index = InstallIndex::load().unwrap_or_default();
                let all_packages: Vec<InstalledApp> = index.entries.values().cloned().collect();
                Box::new(ManualUpdateView::new(all_packages))
            }
            _ => todo!(),
        }
    }

    pub fn handle_action(&mut self, ctx: &egui::Context, action: ViewAction) {
        match action {
            ViewAction::Navigate(route) => {
                let new_view = self.create_view(route);
                let old_view = std::mem::replace(&mut self.view, new_view);
                self.history.push(old_view);
                apply_viewport_builder(ctx, self.view.viewport());
            }

            ViewAction::Back => {
                if let Some(prev) = self.history.pop() {
                    self.view = prev;
                    apply_viewport_builder(ctx, self.view.viewport());
                }
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

            let view_ctx = &mut ViewContext {
                package: self.package.as_mut(),
                shared: &mut self.shared,
                candidates: &self.candidates,
            };

            egui::CentralPanel::default().show(ctx, |ui| {
                self.view.ui(ui, view_ctx, &mut |a| captured = Some(a));
            });

            captured
        };

        if let Some(act) = action {
            self.handle_action(ctx, act);
        }
    }
}
