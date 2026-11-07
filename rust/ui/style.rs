//! Colours, sizes and the small text helpers every part of the window shares.

use crate::Run;
use eframe::egui::Color32;

pub const GREEN: Color32 = Color32::from_rgb(70, 180, 90);
pub const RED: Color32 = Color32::from_rgb(230, 80, 90);
pub const AMBER: Color32 = Color32::from_rgb(230, 170, 50);
pub const GREY: Color32 = Color32::from_rgb(150, 150, 150);
const RUNNING_BLUE: Color32 = Color32::from_rgb(90, 150, 240);

pub const TITLE_SIZE: f32 = 16.0;
pub const SPACE_SMALL: f32 = 4.0;
pub const SPACE_MEDIUM: f32 = 6.0;

const SECONDS_PER_MINUTE: i64 = 60;
const SECONDS_PER_HOUR: i64 = 3600;
const LOG_BYTE_LIMIT: usize = 400_000;

// The colour shown for a run's status; anything unknown is grey.
pub fn status_color(status: &str) -> Color32 {
    match status {
        "ok" => GREEN,
        "failed" => RED,
        "running" => RUNNING_BLUE,
        "interrupted" => AMBER,
        _ => GREY,
    }
}

// The word shown for a run's status.
pub fn status_label(status: &str) -> &'static str {
    match status {
        "ok" => "OK",
        "failed" => "FAILED",
        "running" => "Running",
        "interrupted" => "Interrupted",
        "skipped" => "Skipped",
        _ => "?",
    }
}

/// How long a run took, or has taken so far if it is still going.
pub fn duration_text(past_run: &Run) -> String {
    let finished_at = if past_run.finished_at == 0 { crate::n_now() } else { past_run.finished_at };
    let seconds = (finished_at - past_run.started_at).max(0);

    if seconds >= SECONDS_PER_HOUR {
        format!("{}h {}m", seconds / SECONDS_PER_HOUR, (seconds % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE)
    } else if seconds >= SECONDS_PER_MINUTE {
        format!("{}m {}s", seconds / SECONDS_PER_MINUTE, seconds % SECONDS_PER_MINUTE)
    } else {
        format!("{seconds}s")
    }
}

/// The end of a run's log; a huge log is cut from the front so the window stays quick.
pub fn read_log(run_id: i64) -> String {
    match std::fs::read(crate::log_file_path(run_id)) {
        Ok(bytes) => {
            let start = bytes.len().saturating_sub(LOG_BYTE_LIMIT);
            let mut text = String::from_utf8_lossy(&bytes[start..]).to_string();

            if start > 0 {
                text.insert_str(0, "[... earlier output cut off ...]\n");
            }

            text
        }

        Err(error) => format!("(could not read the log file for this run: {error})"),
    }
}
