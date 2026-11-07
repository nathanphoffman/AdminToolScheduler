//! The tray icon: a coloured dot that opens the window. Without a tray the service still runs, just without an icon.

#[path = "tray/icon.rs"]
mod icon;

#[path = "tray/job_tray.rs"]
mod job_tray;

#[path = "tray/window.rs"]
mod window;

use icon::TrayStatus;
use job_tray::JobTray;
use ksni::blocking::{Handle, TrayMethods};
use std::sync::Mutex;

static TRAY_HANDLE: Mutex<Option<Handle<JobTray>>> = Mutex::new(None);

pub fn start() {
    match (JobTray { status: TrayStatus::Idle }).spawn() {
        Ok(handle) => *TRAY_HANDLE.lock().unwrap() = Some(handle),
        Err(error) => eprintln!("Could not create the tray icon: {error}"),
    }
}

/// Sets the dot's colour from a status word: "idle", "running" or "failed".
pub fn update(status_name: &str) {
    if let Some(handle) = TRAY_HANDLE.lock().unwrap().as_ref() {
        let status = TrayStatus::from_name(status_name);

        // Hands the new status to the tray's own thread, which redraws the dot.
        handle.update(|tray| tray.status = status);
    }
}
