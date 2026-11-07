//! What the window shows right now: the data loaded from the database, and which run's log is open.

use crate::{Database, Job, Run};
use std::time::{Duration, Instant};

const HEARTBEAT_FRESH_SECONDS: i64 = 15;
const REFRESH_INTERVAL: Duration = Duration::from_secs(1);
const FORCE_FIRST_LOAD: Duration = Duration::from_secs(10);

/// A copy of the jobs, runs and service heartbeat, reloaded about once a second.
pub struct Snapshot {
    pub jobs: Vec<Job>,
    pub runs: Vec<Run>,
    pub daemon_alive: bool,
    last_refresh: Instant,
}

impl Snapshot {
    pub fn empty() -> Self {
        Snapshot { jobs: vec![], runs: vec![], daemon_alive: false, last_refresh: Instant::now() - FORCE_FIRST_LOAD }
    }

    /// Makes the next frame reload everything, so a click shows its result without reloading inside the click handler.
    pub fn mark_stale(&mut self) {
        self.last_refresh = Instant::now() - FORCE_FIRST_LOAD;
    }

    pub fn is_stale(&self) -> bool {
        self.last_refresh.elapsed() > REFRESH_INTERVAL
    }

    pub fn reload(&mut self, conn: &Database, filter_job: Option<i64>) {
        self.last_refresh = Instant::now();
        self.jobs = crate::db_list_jobs(conn.clone());
        self.runs = crate::db_list_runs(conn.clone(), filter_job.unwrap_or(0));

        let heartbeat = crate::db_get_meta(conn.clone(), "heartbeat".to_string());
        self.daemon_alive = heartbeat > 0 && crate::n_now() - heartbeat < HEARTBEAT_FRESH_SECONDS;
    }

    /// The newest run of a job that actually ran (skipped ones do not count).
    pub fn latest_run_of(&self, job_id: i64) -> Option<&Run> {
        self.runs.iter().find(|past_run| past_run.job_id == job_id && past_run.status != "skipped")
    }

    pub fn run_by_id(&self, run_id: i64) -> Option<&Run> {
        self.runs.iter().find(|past_run| past_run.run_id == run_id)
    }
}

/// The History tab's choices: which job to filter by, which run is open, and that run's log text.
pub struct View {
    pub filter_job: Option<i64>,
    pub selected_run: Option<i64>,
    pub log_text: String,
    log_state: Option<(i64, bool)>, // past_run id and whether it had finished when the log was read
}

impl View {
    pub fn empty() -> Self {
        View { filter_job: None, selected_run: None, log_text: String::new(), log_state: None }
    }

    pub fn select_run(&mut self, run_id: i64) {
        self.selected_run = Some(run_id);
        self.log_state = None;
    }

    pub fn refresh_log(&mut self, snapshot: &Snapshot) {
        let Some(run_id) = self.selected_run else { return };
        let finished = snapshot.run_by_id(run_id).map(|past_run| past_run.status != "running").unwrap_or(true);

        // A finished log is read once; a running one is read again every refresh to follow its output.
        if self.log_state != Some((run_id, finished)) || !finished {
            self.log_text = super::style::read_log(run_id);
            self.log_state = Some((run_id, finished));
        }
    }
}
