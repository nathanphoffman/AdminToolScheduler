//! The state behind the job editor window, and the checks that decide whether it can be saved.

use crate::Job;

const MINUTES_PER_HOUR: i64 = 60;
const HOURS_PER_DAY: i64 = 24;
const DEFAULT_HOUR: &str = "03";
const DEFAULT_MINUTE: &str = "00";
pub const DAILY: &str = "daily";
pub const INTERVAL: &str = "interval";
const CLOCK_ERROR: &str = "Enter a time like 03:30 (hour 0-23, minute 0-59)";
const INTERVAL_ERROR: &str = "Enter a whole number of minutes or hours (1 or more)";

/// The job being edited plus the text boxes for its schedule, which are kept as text until saved.
pub struct Editor {
    pub editing: Option<Job>,
    pub hour: String,
    pub minute: String,
    pub number: String,
    pub in_hours: bool,
    pub error: Option<String>,
}

// Turns the hour and minute boxes into minutes after midnight, or explains what is wrong.
fn clock_minutes(editor: &Editor) -> Result<i64, String> {
    match (editor.hour.trim().parse::<i64>(), editor.minute.trim().parse::<i64>()) {
        (Ok(hour), Ok(minute)) if (0..HOURS_PER_DAY).contains(&hour) && (0..MINUTES_PER_HOUR).contains(&minute) => {
            Ok(hour * MINUTES_PER_HOUR + minute)
        }

        _ => Err(CLOCK_ERROR.to_string()),
    }
}

// Turns the 'every N' box into minutes; N must be a whole number of 1 or more.
fn interval_minutes(editor: &Editor) -> Result<i64, String> {
    match editor.number.trim().parse::<i64>() {
        Ok(number) if number >= 1 => Ok(if editor.in_hours { number * MINUTES_PER_HOUR } else { number }),
        _ => Err(INTERVAL_ERROR.to_string()),
    }
}

impl Editor {
    pub fn blank() -> Self {
        Editor { editing: None, hour: DEFAULT_HOUR.into(), minute: DEFAULT_MINUTE.into(), number: "1".into(), in_hours: true, error: None }
    }

    /// Opens the editor on a job, filling the schedule boxes from its saved schedule.
    pub fn begin(&mut self, target_job: Job) {
        *self = Editor::blank();

        if target_job.sched_kind == DAILY {
            self.hour = format!("{:02}", target_job.sched_value / MINUTES_PER_HOUR);
            self.minute = format!("{:02}", target_job.sched_value % MINUTES_PER_HOUR);
        } else if target_job.sched_value % MINUTES_PER_HOUR == 0 {
            self.number = (target_job.sched_value / MINUTES_PER_HOUR).to_string();
        } else {
            self.number = target_job.sched_value.to_string();
            self.in_hours = false;
        }

        self.editing = Some(target_job);
    }

    /// The schedule as a number: minutes after midnight for daily jobs, minutes between runs otherwise.
    pub fn schedule_minutes(&self, kind: &str) -> Result<i64, String> {
        if kind == DAILY { clock_minutes(self) } else { interval_minutes(self) }
    }

    pub fn summary(&self, target_job: &Job) -> String {
        match self.schedule_minutes(&target_job.sched_kind) {
            Ok(minutes) => {
                let mut shown = target_job.clone();
                shown.sched_value = minutes;

                format!("Currently set to: {}", crate::schedule_text(shown))
            }

            Err(message) => format!("Currently set to: ({message})"),
        }
    }

    /// The job with its schedule filled in, or the reason it cannot be saved yet.
    pub fn checked_job(&self, target_job: &Job) -> Result<Job, String> {
        let minutes = self.schedule_minutes(&target_job.sched_kind)?;

        if target_job.name.trim().is_empty() || target_job.command.trim().is_empty() {
            return Err("A name and a command are required".to_string());
        }

        let mut checked = target_job.clone();
        checked.sched_value = minutes;

        Ok(checked)
    }
}
