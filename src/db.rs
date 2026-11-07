use anyhow::Result;
use chrono::{Local, TimeZone};
use rusqlite::{params, Connection, OptionalExtension};
use std::path::PathBuf;

pub fn data_dir() -> PathBuf {
    let d = dirs::data_dir().unwrap_or_else(|| PathBuf::from(".")).join("jobrunner");
    let _ = std::fs::create_dir_all(d.join("logs"));
    d
}

pub fn log_path(run_id: i64) -> PathBuf {
    data_dir().join("logs").join(format!("{run_id}.log"))
}

pub fn open() -> Result<Connection> {
    let conn = Connection::open(data_dir().join("jobrunner.db"))?;
    conn.busy_timeout(std::time::Duration::from_secs(5))?;
    conn.execute_batch(
        "PRAGMA journal_mode = WAL;
         CREATE TABLE IF NOT EXISTS jobs (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            cwd TEXT NOT NULL,
            command TEXT NOT NULL,
            sched_kind TEXT NOT NULL,      -- 'daily' or 'interval'
            sched_value INTEGER NOT NULL,  -- daily: minutes after midnight, interval: minutes
            enabled INTEGER NOT NULL DEFAULT 1,
            updated_at INTEGER NOT NULL DEFAULT 0
         );
         CREATE TABLE IF NOT EXISTS runs (
            id INTEGER PRIMARY KEY,
            job_id INTEGER NOT NULL,
            job_name TEXT NOT NULL,
            started_at INTEGER NOT NULL,
            finished_at INTEGER,
            status TEXT NOT NULL,          -- running, ok, failed, interrupted, skipped
            exit_code INTEGER,
            trigger_kind TEXT NOT NULL,    -- schedule, manual
            note TEXT
         );
         CREATE TABLE IF NOT EXISTS run_requests (
            id INTEGER PRIMARY KEY,
            job_id INTEGER NOT NULL
         );
         CREATE TABLE IF NOT EXISTS meta (key TEXT PRIMARY KEY, value INTEGER NOT NULL);",
    )?;
    Ok(conn)
}

#[derive(Clone, Debug)]
pub struct Job {
    pub id: i64,
    pub name: String,
    pub cwd: String,
    pub command: String,
    pub sched_kind: String,
    pub sched_value: i64,
    pub enabled: bool,
    pub updated_at: i64,
}

impl Job {
    pub fn blank() -> Job {
        Job {
            id: 0,
            name: String::new(),
            cwd: "~".into(),
            command: String::new(),
            sched_kind: "interval".into(),
            sched_value: 60,
            enabled: false,
            updated_at: 0,
        }
    }

    pub fn schedule_text(&self) -> String {
        if self.sched_kind == "daily" {
            format!("Daily at {:02}:{:02}", self.sched_value / 60, self.sched_value % 60)
        } else if self.sched_value % 60 == 0 {
            let h = self.sched_value / 60;
            if h == 1 { "Every hour".into() } else { format!("Every {h} hours") }
        } else {
            format!("Every {} min", self.sched_value)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Run {
    pub id: i64,
    pub job_id: i64,
    pub job_name: String,
    pub started_at: i64,
    pub finished_at: Option<i64>,
    pub status: String,
    pub exit_code: Option<i64>,
    pub trigger_kind: String,
    pub note: Option<String>,
}

pub fn now() -> i64 {
    Local::now().timestamp()
}

pub fn fmt_time(ts: i64) -> String {
    Local
        .timestamp_opt(ts, 0)
        .single()
        .map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_default()
}

pub fn list_jobs(c: &Connection) -> Result<Vec<Job>> {
    let mut st = c.prepare(
        "SELECT id,name,cwd,command,sched_kind,sched_value,enabled,updated_at FROM jobs ORDER BY id",
    )?;
    let rows = st.query_map([], |r| {
        Ok(Job {
            id: r.get(0)?,
            name: r.get(1)?,
            cwd: r.get(2)?,
            command: r.get(3)?,
            sched_kind: r.get(4)?,
            sched_value: r.get(5)?,
            enabled: r.get::<_, i64>(6)? != 0,
            updated_at: r.get(7)?,
        })
    })?;
    Ok(rows.collect::<std::result::Result<_, _>>()?)
}

pub fn get_job(c: &Connection, id: i64) -> Result<Option<Job>> {
    Ok(list_jobs(c)?.into_iter().find(|j| j.id == id))
}

pub fn save_job(c: &Connection, j: &Job) -> Result<()> {
    if j.id == 0 {
        c.execute(
            "INSERT INTO jobs (name,cwd,command,sched_kind,sched_value,enabled,updated_at) VALUES (?,?,?,?,?,?,?)",
            params![j.name, j.cwd, j.command, j.sched_kind, j.sched_value, j.enabled as i64, now()],
        )?;
    } else {
        c.execute(
            "UPDATE jobs SET name=?,cwd=?,command=?,sched_kind=?,sched_value=?,enabled=?,updated_at=? WHERE id=?",
            params![j.name, j.cwd, j.command, j.sched_kind, j.sched_value, j.enabled as i64, now(), j.id],
        )?;
    }
    Ok(())
}

pub fn delete_job(c: &Connection, id: i64) -> Result<()> {
    c.execute("DELETE FROM jobs WHERE id=?", [id])?;
    Ok(())
}

pub fn list_runs(c: &Connection, job_id: Option<i64>, limit: i64) -> Result<Vec<Run>> {
    let mut st = c.prepare(
        "SELECT id,job_id,job_name,started_at,finished_at,status,exit_code,trigger_kind,note
         FROM runs WHERE (?1 IS NULL OR job_id = ?1) ORDER BY id DESC LIMIT ?2",
    )?;
    let rows = st.query_map(params![job_id, limit], |r| {
        Ok(Run {
            id: r.get(0)?,
            job_id: r.get(1)?,
            job_name: r.get(2)?,
            started_at: r.get(3)?,
            finished_at: r.get(4)?,
            status: r.get(5)?,
            exit_code: r.get(6)?,
            trigger_kind: r.get(7)?,
            note: r.get(8)?,
        })
    })?;
    Ok(rows.collect::<std::result::Result<_, _>>()?)
}

pub fn insert_run(c: &Connection, job: &Job, status: &str, trigger: &str, note: Option<&str>) -> Result<i64> {
    let t = now();
    let finished = if status == "running" { None } else { Some(t) };
    c.execute(
        "INSERT INTO runs (job_id,job_name,started_at,finished_at,status,trigger_kind,note) VALUES (?,?,?,?,?,?,?)",
        params![job.id, job.name, t, finished, status, trigger, note],
    )?;
    Ok(c.last_insert_rowid())
}

pub fn finish_run(c: &Connection, id: i64, status: &str, code: Option<i32>, note: Option<&str>) -> Result<()> {
    c.execute(
        "UPDATE runs SET finished_at=?, status=?, exit_code=?, note=? WHERE id=?",
        params![now(), status, code, note, id],
    )?;
    Ok(())
}

pub fn request_run(c: &Connection, job_id: i64) -> Result<()> {
    c.execute("INSERT INTO run_requests (job_id) VALUES (?)", [job_id])?;
    Ok(())
}

pub fn take_run_requests(c: &Connection) -> Result<Vec<i64>> {
    let mut st = c.prepare("SELECT job_id FROM run_requests ORDER BY id")?;
    let ids: Vec<i64> = st.query_map([], |r| r.get(0))?.collect::<std::result::Result<_, _>>()?;
    c.execute("DELETE FROM run_requests", [])?;
    Ok(ids)
}

pub fn set_meta(c: &Connection, key: &str, value: i64) -> Result<()> {
    c.execute("INSERT OR REPLACE INTO meta (key,value) VALUES (?,?)", params![key, value])?;
    Ok(())
}

pub fn get_meta(c: &Connection, key: &str) -> Option<i64> {
    c.query_row("SELECT value FROM meta WHERE key=?", [key], |r| r.get(0)).optional().ok().flatten()
}

pub fn mark_interrupted(c: &Connection) -> Result<()> {
    c.execute(
        "UPDATE runs SET status='interrupted', finished_at=?, note='The job runner stopped while this was running' WHERE status='running'",
        [now()],
    )?;
    Ok(())
}

pub fn seed_jobs(c: &Connection) -> Result<()> {
    let n: i64 = c.query_row("SELECT COUNT(*) FROM jobs", [], |r| r.get(0))?;
    if n > 0 {
        return Ok(());
    }
    // Default jobs and schedules, all switched on.
    let seeds = [
        ("Proton Drive sync", "~/Code/ProtonDriveSync", "npm start -- --once", "daily", 2 * 60, true),
        ("GraphData prod sync", "~/Code/GraphData", "just sync-prod", "interval", 60, true),
        (
            "Line quality check + sync",
            "~/Code/GraphData",
            "just refresh-local-line-quality\njust sync-prod",
            "daily",
            60,
            true,
        ),
    ];
    for (name, cwd, cmd, kind, val, enabled) in seeds {
        save_job(
            c,
            &Job {
                id: 0,
                name: name.into(),
                cwd: cwd.into(),
                command: cmd.into(),
                sched_kind: kind.into(),
                sched_value: val,
                enabled,
                updated_at: 0,
            },
        )?;
    }
    Ok(())
}
