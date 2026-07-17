pub mod job;
pub mod routing;
pub mod state;

use crate::app::job::{InstallJob, JobOutcome};
use crate::app::routing::{Route, ViewAction};
use crate::app::state::AppData;
use crate::state::config::Config;
use crate::state::index::{InstallIndex, InstalledApp};
use crate::state::persistable::Persistable;
use crate::ui::View;
use crate::ui::app_list_view::AppListView;
use crate::ui::dialogs::{show_error_message, show_info_message, show_warning_message};
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
            Route::Setup => {
                let config = Config::load().unwrap_or_default();
                Box::new(SetupView::new(config.is_installed()))
            }

            Route::Install => {
                let config = Config::load().unwrap_or_default();
                let index = InstallIndex::load().unwrap_or_default();
                Box::new(InstallView::new(!index.apps(config.self_uuid.as_deref()).is_empty()))
            }

            Route::Update(target) => Box::new(UpdateView::new(target.clone())),

            Route::ManualUpdate => {
                let config = Config::load().unwrap_or_default();
                let index = InstallIndex::load().unwrap_or_default();
                Box::new(ManualUpdateView::new(index.apps(config.self_uuid.as_deref())))
            }

            Route::AppList => {
                let config = Config::load().unwrap_or_default();
                let index = InstallIndex::load().unwrap_or_default();
                Box::new(AppListView::new(index.apps(config.self_uuid.as_deref())))
            }
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
        }
    }

    pub fn viewport(&self) -> egui::ViewportBuilder {
        self.view.viewport()
    }

    fn poll_job(&mut self, ctx: &egui::Context) {
        let Some(job) = &self.data.job else { return };

        match job.rx.try_recv() {
            Err(std::sync::mpsc::TryRecvError::Empty) => {}

            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                self.data.job = None;
                show_error_message("Installation failed: worker thread terminated unexpectedly.");
            }

            Ok((package, result)) => {
                let job = self.data.job.take().expect("job checked above");
                self.data.package = Some(package);
                Self::finish_job(ctx, &job, result);
            }
        }
    }

    fn finish_job(ctx: &egui::Context, job: &InstallJob, result: anyhow::Result<JobOutcome>) {
        match result {
            Ok(outcome) => {
                let mut entry = InstalledApp::from(&job.candidate);
                entry.shortcuts = outcome.shortcuts;
                entry.fingerprint = outcome.fingerprint;

                let mut index = InstallIndex::load().unwrap_or_default();
                index.add_entry(&outcome.uuid, entry);
                if let Err(e) = index.save() {
                    show_error_message(&format!("Failed to save index: {}", e));
                }

                if job.remove_source
                    && let Err(e) = std::fs::remove_file(&job.source)
                {
                    show_warning_message(&format!("Failed to remove package: {}", e));
                }

                show_info_message(if job.is_update {
                    "Application updated successfully."
                } else {
                    "Application installed successfully."
                });

                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }

            Err(e) => {
                let verb = if job.is_update { "update" } else { "install" };
                show_error_message(&format!("Failed to {}! {}", verb, e));
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        self.poll_job(ctx);

        if let Some(job) = &self.data.job {
            let message = if job.is_update { "Updating…" } else { "Installing…" };

            egui::CentralPanel::default().show(ctx, |ui| {
                let offset = ui.available_height() * 0.35;
                ui.vertical_centered(|ui| {
                    ui.add_space(offset);
                    ui.add(egui::Spinner::new().size(28.0));
                    ui.add_space(8.0);
                    ui.label(message);
                });
            });
            return;
        }

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
