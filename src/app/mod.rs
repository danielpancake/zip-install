pub mod routing;
pub mod state;

use crate::app::routing::{Route, ViewAction};
use crate::app::state::AppData;
use crate::state::index::{InstallIndex, InstalledApp};
use crate::state::persistable::Persistable;
use crate::ui::View;
use crate::ui::install_view::InstallView;
use crate::ui::manual_update_view::ManualUpdateView;
use crate::ui::setup_view::SetupView;
use crate::ui::update_view::UpdateView;
use crate::ui::viewport::apply_viewport_builder;

use eframe::egui;

pub struct App {
    data: AppData,
    route: Route,
    view: Box<dyn View>,
    history: Vec<Route>,
}

impl App {
    pub fn new(data: AppData, route: Route, view: Box<dyn View>) -> Self {
        Self {
            data,
            route,
            view,
            history: Vec::new(),
        }
    }

    fn create_view(&self, route: &Route) -> Box<dyn View> {
        match route {
            Route::Setup => Box::new(SetupView::new(self.data.is_installed)),

            Route::Install => {
                let index = InstallIndex::load().unwrap_or_default();
                Box::new(InstallView::new(!index.entries.is_empty()))
            }

            Route::Update(target) => Box::new(UpdateView::new(target.clone())),

            Route::ManualUpdate => {
                let index = InstallIndex::load().unwrap_or_default();
                let all_packages: Vec<InstalledApp> = index
                    .entries
                    .iter()
                    .map(|(uuid, app)| {
                        let mut app = app.clone();
                        app.uuid = uuid.clone();
                        app
                    })
                    .collect();
                Box::new(ManualUpdateView::new(all_packages))
            }

            Route::AppList => todo!(),
        }
    }

    pub fn handle_action(&mut self, ctx: &egui::Context, action: ViewAction) {
        match action {
            ViewAction::Navigate(new_route) => {
                self.history.push(self.route.clone());
                self.view = self.create_view(&new_route);
                self.route = new_route;
                apply_viewport_builder(ctx, self.view.viewport());
            }

            ViewAction::Back => {
                if let Some(prev_route) = self.history.pop() {
                    self.view = self.create_view(&prev_route);
                    self.route = prev_route;
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

            let App { data, view, .. } = self;

            egui::CentralPanel::default().show(ctx, |ui| {
                view.ui(ui, data, &mut |a| captured = Some(a));
            });

            captured
        };

        if let Some(act) = action {
            self.handle_action(ctx, act);
        }
    }
}
