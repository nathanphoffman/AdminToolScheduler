use crate::db::{self, Job};
use crate::tray;
use anyhow::{bail, Result};
use chrono::{DateTime, Duration, Local, NaiveTime, TimeZone};
use std::collections::{HashMap, HashSet};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::Duration as StdDuration;

/// A run that is more than this many seconds late is treated as "the computer was asleep" and skipped.
const LATE_LIMIT_SECS: i64 = 90;

pub fn expand_home(p: &str) -> String {
    if let Some(rest) = p.strip_prefix("~") {
        if let Some(h) = dirs::home_dir() {
            return format!("{}{}", h.display(), rest);
        }
    }
    p.to_string()
}

fn next_due(job: &Job, from: DateTime<Local>) -> DateTime<Local> {
    if job.sched_kind == "daily" {
        let t = NaiveTime::from_hms_opt((job.sched_value / 60) as u32 % 24, (job.sched_value % 60) as u32, 0).unwrap();
        let today = from.date_naive().and_time(t);
        let mut cand = Local.from_local_datetime(&today).earliest().unwrap_or(from);
        if cand <= from {
            cand = Local
                .from_local_datetime(&(today + Duration::days(1)))
                .earliest()
                .unwrap_or(from + Duration::days(1));
        }
        cand
    } else {
        from + Duration::minutes(job.sched_value.max(1))
    }
}

pub fn run_daemon() -> Result<()> {
    // Only one background service at a time.
    let lock = OpenOptions::new().create(true).write(true).truncate(false).open(db::data_dir().join("daemon.lock"))?;
    if lock.try_lock().is_err() {
        bail!("The job runner is already running.");
    }

    let conn = db::open()?;
    db::mark_interrupted(&conn)?;
    db::seed_jobs(&conn)?;

    let tray_state = tray::start();
    let running: Arc<Mutex<HashSet<i64>>> = Arc::new(Mutex::new(HashSet::new()));
    let mut due: HashMap<i64, (i64, DateTime<Local>)> = HashMap::new();
    let mut last_failed = false;

    loop {
        let _ = db::set_meta(&conn, "heartbeat", db::now());
        let jobs = db::list_jobs(&conn).unwrap_or_default();
        let now = Local::now();

        // Forget schedules of deleted jobs and recompute any that were edited.
        due.retain(|id, _| jobs.iter().any(|j| j.id == *id));
        for job in &jobs {
            let stale = due.get(&job.id).map(|(u, _)| *u != job.updated_at).unwrap_or(true);
            if stale {
                due.insert(job.id, (job.updated_at, next_due(job, now)));
            }
        }

        // Scheduled runs.
        for job in jobs.iter().filter(|j| j.enabled) {
            let (updated, when) = due[&job.id];
            if now < when {
                continue;
            }
            due.insert(job.id, (updated, next_due(job, now)));
            let late = (now - when).num_seconds();
            if late > LATE_LIMIT_SECS {
                let _ = db::insert_run(&conn, job, "skipped", "schedule", Some("Skipped: the computer was asleep or off at the scheduled time"));
            } else {
                start_job(job, "schedule", &running);
            }
        }

        // "Run now" clicks from the window.
        for id in db::take_run_requests(&conn).unwrap_or_default() {
            if let Some(job) = jobs.iter().find(|j| j.id == id) {
                start_job(job, "manual", &running);
            }
        }

        // Tray icon colour: running, last run failed, or idle.
        if let Ok(runs) = db::list_runs(&conn, None, 50) {
            last_failed = runs
                .iter()
                .find(|r| r.status != "skipped" && r.status != "running")
                .map(|r| r.status == "failed")
                .unwrap_or(false);
        }
        let n_running = running.lock().unwrap().len();
        tray_state.update(n_running, last_failed);

        std::thread::sleep(StdDuration::from_secs(2));
    }
}

fn start_job(job: &Job, trigger: &'static str, running: &Arc<Mutex<HashSet<i64>>>) {
    let job = job.clone();
    {
        let mut r = running.lock().unwrap();
        if r.contains(&job.id) {
            if let Ok(c) = db::open() {
                let _ = db::insert_run(&c, &job, "skipped", trigger, Some("Skipped: the previous run was still going"));
            }
            return;
        }
        r.insert(job.id);
    }
    let running = running.clone();
    std::thread::spawn(move || {
        if let Err(e) = execute(&job, trigger) {
            eprintln!("job {} failed to run: {e:#}", job.name);
        }
        running.lock().unwrap().remove(&job.id);
    });
}

fn execute(job: &Job, trigger: &str) -> Result<()> {
    let conn = db::open()?;
    let run_id = db::insert_run(&conn, job, "running", trigger, None)?;
    let mut log = std::fs::File::create(db::log_path(run_id))?;
    let cwd = expand_home(&job.cwd);
    writeln!(log, "[{}] Starting \"{}\" in {}", db::fmt_time(db::now()), job.name, cwd)?;
    writeln!(log, "$ {}\n", job.command.replace('\n', "\n$ "))?;

    // stderr is folded into stdout so the log keeps the real order of messages.
    // `set -e` stops a multi-line job at the first failing line.
    let script = format!("exec 2>&1\nset -e\n{}\n", job.command);
    let child = Command::new("bash")
        .arg("-c")
        .arg(&script)
        .current_dir(&cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .spawn();

    let mut child = match child {
        Ok(c) => c,
        Err(e) => {
            let msg = format!("Could not start the job: {e} (folder: {cwd})");
            writeln!(log, "{msg}")?;
            db::finish_run(&conn, run_id, "failed", None, Some(&msg))?;
            return Ok(());
        }
    };

    let mut out = child.stdout.take().unwrap();
    let mut buf = [0u8; 4096];
    loop {
        match out.read(&mut buf) {
            Ok(0) | Err(_) => break,
            Ok(n) => {
                log.write_all(&buf[..n])?;
                log.flush()?;
            }
        }
    }
    let status = child.wait()?;
    let code = status.code();
    if status.success() {
        writeln!(log, "\n[{}] Finished OK", db::fmt_time(db::now()))?;
        db::finish_run(&conn, run_id, "ok", code, None)?;
    } else {
        let why = match code {
            Some(c) => format!("Failed: the command exited with code {c}"),
            None => "Failed: the command was stopped by a signal".to_string(),
        };
        writeln!(log, "\n[{}] {why}", db::fmt_time(db::now()))?;
        db::finish_run(&conn, run_id, "failed", code, Some(&why))?;
    }
    Ok(())
}
