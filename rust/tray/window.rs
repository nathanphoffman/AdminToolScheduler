//! Opening the window. It is a separate process, so a crash in the window never takes the scheduler down.

use std::process::{Child, Command};
use std::sync::Mutex;

static WINDOW_PROCESS: Mutex<Option<Child>> = Mutex::new(None);

/// Starts the window unless one is already open.
pub fn open_window() {
    let mut open = WINDOW_PROCESS.lock().unwrap();

    if let Some(child) = open.as_mut() {
        if matches!(child.try_wait(), Ok(None)) {
            return;
        }
    }

    match std::env::current_exe().and_then(|exe| Command::new(exe).arg("ui").spawn()) {
        Ok(child) => *open = Some(child),
        Err(error) => eprintln!("Could not open the window: {error}"),
    }
}
