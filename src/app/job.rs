use std::path::PathBuf;
use std::sync::mpsc::{Receiver, channel};

use anyhow::Result;
use eframe::egui;

use crate::core::fingerprint::Fingerprint;
use crate::core::installer;
use crate::package::{Candidate, Package};
use crate::state::index::{InstalledApp, StoredFingerprint};

pub struct JobOutcome {
    pub uuid: String,
    pub shortcuts: Vec<String>,
    pub fingerprint: StoredFingerprint,
}

type JobResult = (Box<dyn Package>, Result<JobOutcome>);

pub struct InstallJob {
    pub rx: Receiver<JobResult>,
    pub candidate: Candidate,
    pub is_update: bool,
    pub remove_source: bool,
    pub source: PathBuf,
}

pub fn spawn(
    mut package: Box<dyn Package>,
    candidate: Candidate,
    target: Option<&InstalledApp>,
    create_desktop_shortcut: bool,
    create_app_launcher_shortcut: bool,
    remove_source: bool,
    ctx: egui::Context,
) -> InstallJob {
    let (tx, rx) = channel();

    let source = package.source().to_path_buf();
    let is_update = target.is_some();
    let target_uuid = target.map(|t| t.uuid.clone());
    let old_shortcuts: Vec<PathBuf> = target
        .map(|t| t.shortcuts.iter().map(PathBuf::from).collect())
        .unwrap_or_default();
    let job_candidate = candidate.clone();

    std::thread::spawn(move || {
        let result = run(
            package.as_mut(),
            &job_candidate,
            target_uuid.as_deref(),
            &old_shortcuts,
            create_desktop_shortcut,
            create_app_launcher_shortcut,
        );
        let _ = tx.send((package, result));
        ctx.request_repaint();
    });

    InstallJob {
        rx,
        candidate,
        is_update,
        remove_source,
        source,
    }
}

fn run(
    package: &mut dyn Package,
    candidate: &Candidate,
    target_uuid: Option<&str>,
    old_shortcuts: &[PathBuf],
    create_desktop_shortcut: bool,
    create_app_launcher_shortcut: bool,
) -> Result<JobOutcome> {
    let fingerprint = StoredFingerprint::from(&Fingerprint::from_package(package));

    let (uuid, shortcuts) = installer::install(
        package,
        candidate,
        target_uuid,
        old_shortcuts,
        create_desktop_shortcut,
        create_app_launcher_shortcut,
    )?;

    Ok(JobOutcome {
        uuid,
        shortcuts: shortcuts.iter().map(|p| p.to_string_lossy().into_owned()).collect(),
        fingerprint,
    })
}
