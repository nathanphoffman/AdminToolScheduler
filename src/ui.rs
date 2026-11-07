use crate::db::{self, Job, Run};
use eframe::egui::{self, Color32, RichText};
use rusqlite::Connection;
use std::time::{Duration, Instant};

#[derive(PartialEq)]
enum Tab {
    Jobs,
    History,
}

pub struct App {
    conn: Connection,
    tab: Tab,
    jobs: Vec<Job>,
    runs: Vec<Run>,
    last_refresh: Instant,
    daemon_alive: bool,
    filter_job: Option<i64>,
    selected_run: Option<i64>,
    log_text: String,
    log_for: Option<(i64, bool)>, // run id, was it finished when read
    editing: Option<Job>,
    edit_hour: String,
    edit_min: String,
    error: Option<String>,
}

const GREEN: Color32 = Color32::from_rgb(70, 180, 90);
const RED: Color32 = Color32::from_rgb(230, 80, 90);
const AMBER: Color32 = Color32::from_rgb(230, 170, 50);
const GREY: Color32 = Color32::from_rgb(150, 150, 150);

fn status_color(s: &str) -> Color32 {
    match s {
        "ok" => GREEN,
        "failed" => RED,
        "running" => Color32::from_rgb(90, 150, 240),
        "interrupted" => AMBER,
        _ => GREY,
    }
}

fn status_label(s: &str) -> &'static str {
    match s {
        "ok" => "OK",
        "failed" => "FAILED",
        "running" => "Running",
        "interrupted" => "Interrupted",
        "skipped" => "Skipped",
        _ => "?",
    }
}

fn duration_text(r: &Run) -> String {
    let end = r.finished_at.unwrap_or_else(db::now);
    let s = (end - r.started_at).max(0);
    if s >= 3600 {
        format!("{}h {}m", s / 3600, (s % 3600) / 60)
    } else if s >= 60 {
        format!("{}m {}s", s / 60, s % 60)
    } else {
        format!("{s}s")
    }
}

impl App {
    pub fn new(conn: Connection) -> Self {
        let mut a = App {
            conn,
            tab: Tab::Jobs,
            jobs: vec![],
            runs: vec![],
            last_refresh: Instant::now() - Duration::from_secs(10),
            daemon_alive: false,
            filter_job: None,
            selected_run: None,
            log_text: String::new(),
            log_for: None,
            editing: None,
            edit_hour: String::new(),
            edit_min: String::new(),
            error: None,
        };
        a.refresh();
        a
    }

    fn refresh(&mut self) {
        self.last_refresh = Instant::now();
        self.jobs = db::list_jobs(&self.conn).unwrap_or_default();
        self.runs = db::list_runs(&self.conn, self.filter_job, 300).unwrap_or_default();
        self.daemon_alive = db::get_meta(&self.conn, "heartbeat").map(|h| db::now() - h < 15).unwrap_or(false);

        if let Some(id) = self.selected_run {
            let finished = self.runs.iter().find(|r| r.id == id).map(|r| r.status != "running").unwrap_or(true);
            if self.log_for != Some((id, finished)) {
                self.log_text = read_log(id);
                self.log_for = Some((id, finished));
            } else if !finished {
                self.log_text = read_log(id);
            }
        }
    }

    fn last_run_of(&self, job_id: i64) -> Option<&Run> {
        self.runs.iter().find(|r| r.job_id == job_id && r.status != "skipped")
    }

    fn jobs_tab(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("+ New job").clicked() {
                self.edit_hour = "03".into();
                self.edit_min = "00".into();
                self.editing = Some(Job::blank());
            }
        });
        ui.add_space(6.0);

        let jobs = self.jobs.clone();
        let mut changed = false;
        egui::ScrollArea::vertical().show(ui, |ui| {
            for job in &jobs {
                ui.group(|ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        let mut enabled = job.enabled;
                        if ui.checkbox(&mut enabled, "").on_hover_text("Run this job on its schedule").changed() {
                            let mut j = job.clone();
                            j.enabled = enabled;
                            let _ = db::save_job(&self.conn, &j);
                            changed = true;
                        }
                        ui.label(RichText::new(&job.name).strong().size(16.0));
                        ui.label(RichText::new(job.schedule_text()).color(GREY));
                        if !job.enabled {
                            ui.label(RichText::new("(off)").color(AMBER));
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("Edit").clicked() {
                                self.edit_hour = format!("{:02}", job.sched_value / 60);
                                self.edit_min = format!("{:02}", job.sched_value % 60);
                                self.editing = Some(job.clone());
                            }
                            if ui.button("Run now").clicked() {
                                let _ = db::request_run(&self.conn, job.id);
                            }
                        });
                    });
                    match self.last_run_of(job.id).cloned() {
                        Some(r) => {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(status_label(&r.status)).color(status_color(&r.status)).strong());
                                ui.label(format!("{} ({})", db::fmt_time(r.started_at), duration_text(&r)));
                                if ui.link("view log").clicked() {
                                    self.tab = Tab::History;
                                    self.filter_job = None;
                                    self.selected_run = Some(r.id);
                                    self.log_for = None;
                                    changed = true;
                                }
                            });
                            if r.status == "failed" {
                                if let Some(n) = &r.note {
                                    ui.label(RichText::new(n).color(RED));
                                }
                            }
                        }
                        None => {
                            ui.label(RichText::new("Never run").color(GREY));
                        }
                    }
                });
            }
        });
        if changed {
            self.refresh();
        }
    }

    fn history_tab(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Job:");
            let current = self
                .filter_job
                .and_then(|id| self.jobs.iter().find(|j| j.id == id).map(|j| j.name.clone()))
                .unwrap_or_else(|| "All jobs".into());
            let mut new_filter = self.filter_job;
            egui::ComboBox::from_id_salt("filter").selected_text(current).show_ui(ui, |ui| {
                ui.selectable_value(&mut new_filter, None, "All jobs");
                for j in &self.jobs {
                    ui.selectable_value(&mut new_filter, Some(j.id), &j.name);
                }
            });
            if new_filter != self.filter_job {
                self.filter_job = new_filter;
                self.refresh();
            }
        });
        ui.add_space(4.0);

        egui::Panel::left("runs_list").resizable(true).default_size(340.0).show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for r in &self.runs.clone() {
                    let selected = self.selected_run == Some(r.id);
                    let resp = ui
                        .selectable_label(selected, "")
                        .rect;
                    // draw the row over the label so we can colour parts of it
                    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(resp));
                    child.horizontal(|ui| {
                        ui.label(RichText::new(status_label(&r.status)).color(status_color(&r.status)).strong());
                        ui.label(&r.job_name);
                    });
                    let click = ui.interact(resp, ui.make_persistent_id(("run", r.id)), egui::Sense::click());
                    ui.label(RichText::new(format!("{}  ({})", db::fmt_time(r.started_at), duration_text(r))).small().color(GREY));
                    if click.clicked() {
                        self.selected_run = Some(r.id);
                        self.log_for = None;
                        self.refresh();
                    }
                    ui.separator();
                }
            });
        });

        match self.selected_run.and_then(|id| self.runs.iter().find(|r| r.id == id).cloned()) {
            Some(r) => {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(&r.job_name).strong().size(16.0));
                    ui.label(RichText::new(status_label(&r.status)).color(status_color(&r.status)).strong());
                    if let Some(c) = r.exit_code {
                        ui.label(format!("exit code {c}"));
                    }
                    ui.label(RichText::new(format!("started by {}", r.trigger_kind)).color(GREY));
                });
                if let Some(n) = &r.note {
                    let color = if r.status == "failed" { RED } else { AMBER };
                    ui.label(RichText::new(n).color(color));
                }
                ui.separator();
                egui::ScrollArea::both().stick_to_bottom(true).auto_shrink([false, false]).show(ui, |ui| {
                    if r.status == "skipped" {
                        ui.label("Nothing ran, so there is no output.");
                    } else {
                        ui.add(egui::Label::new(RichText::new(&self.log_text).monospace()).selectable(true));
                    }
                });
            }
            None => {
                ui.label("Pick a run on the left to see its output.");
            }
        }
    }

    fn edit_window(&mut self, ctx: &egui::Context) {
        let Some(mut job) = self.editing.take() else { return };
        let mut keep_open = true;
        let mut save = false;
        let mut delete = false;
        egui::Window::new(if job.id == 0 { "New job" } else { "Edit job" })
            .collapsible(false)
            .default_width(480.0)
            .show(ctx, |ui| {
                egui::Grid::new("edit_grid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
                    ui.label("Name");
                    ui.text_edit_singleline(&mut job.name);
                    ui.end_row();
                    ui.label("Folder");
                    ui.text_edit_singleline(&mut job.cwd);
                    ui.end_row();
                    ui.label("Command");
                    ui.add(egui::TextEdit::multiline(&mut job.command).desired_rows(3).code_editor().desired_width(380.0));
                    ui.end_row();
                    ui.label("Schedule");
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut job.sched_kind, "daily".to_string(), "Once a day at");
                        ui.selectable_value(&mut job.sched_kind, "interval".to_string(), "Every");
                    });
                    ui.end_row();
                    ui.label("");
                    ui.horizontal(|ui| {
                        if job.sched_kind == "daily" {
                            ui.add(egui::TextEdit::singleline(&mut self.edit_hour).desired_width(30.0));
                            ui.label(":");
                            ui.add(egui::TextEdit::singleline(&mut self.edit_min).desired_width(30.0));
                            ui.label("(24 hour clock)");
                        } else {
                            let mut v = job.sched_value.max(1);
                            ui.add(egui::DragValue::new(&mut v).range(1..=100_000).suffix(" min"));
                            job.sched_value = v;
                            for (label, m) in [("15 min", 15), ("30 min", 30), ("1 hour", 60), ("6 hours", 360)] {
                                if ui.small_button(label).clicked() {
                                    job.sched_value = m;
                                }
                            }
                        }
                    });
                    ui.end_row();
                    ui.label("");
                    ui.checkbox(&mut job.enabled, "Enabled");
                    ui.end_row();
                });
                ui.label(RichText::new("Several lines run one after another and stop at the first one that fails.").small().color(GREY));
                if let Some(e) = &self.error {
                    ui.colored_label(RED, e);
                }
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        save = true;
                    }
                    if ui.button("Cancel").clicked() {
                        keep_open = false;
                    }
                    if job.id != 0 && ui.button(RichText::new("Delete job").color(RED)).clicked() {
                        delete = true;
                    }
                });
            });

        if save {
            if job.sched_kind == "daily" {
                match (self.edit_hour.trim().parse::<i64>(), self.edit_min.trim().parse::<i64>()) {
                    (Ok(h), Ok(m)) if (0..24).contains(&h) && (0..60).contains(&m) => job.sched_value = h * 60 + m,
                    _ => {
                        self.error = Some("Enter a time like 03:30 (hour 0-23, minute 0-59)".into());
                        self.editing = Some(job);
                        return;
                    }
                }
            }
            if job.name.trim().is_empty() || job.command.trim().is_empty() {
                self.error = Some("A name and a command are required".into());
                self.editing = Some(job);
                return;
            }
            let _ = db::save_job(&self.conn, &job);
            self.error = None;
            self.refresh();
        } else if delete {
            let _ = db::delete_job(&self.conn, job.id);
            self.error = None;
            self.refresh();
        } else if keep_open {
            self.editing = Some(job);
        } else {
            self.error = None;
        }
    }
}

fn read_log(run_id: i64) -> String {
    const MAX: usize = 400_000;
    match std::fs::read(db::log_path(run_id)) {
        Ok(bytes) => {
            let start = bytes.len().saturating_sub(MAX);
            let mut s = String::from_utf8_lossy(&bytes[start..]).to_string();
            if start > 0 {
                s.insert_str(0, "[... earlier output cut off ...]\n");
            }
            s
        }
        Err(_) => "(no log file for this run)".into(),
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.last_refresh.elapsed() > Duration::from_secs(1) {
            self.refresh();
        }
        ui.ctx().request_repaint_after(Duration::from_secs(1));

        egui::Panel::top("top").show(ui, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.tab, Tab::Jobs, "Jobs");
                ui.selectable_value(&mut self.tab, Tab::History, "History");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.daemon_alive {
                        ui.label(RichText::new("● Background service running").color(GREEN));
                    } else {
                        ui.label(RichText::new("● Background service NOT running - nothing will be scheduled").color(RED));
                    }
                });
            });
            ui.add_space(4.0);
        });

        match self.tab {
            Tab::Jobs => self.jobs_tab(ui),
            Tab::History => self.history_tab(ui),
        }
        let ctx = ui.ctx().clone();
        self.edit_window(&ctx);
    }
}

pub fn run() -> anyhow::Result<()> {
    let conn = db::open()?;
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([960.0, 620.0]).with_title("Job Runner"),
        ..Default::default()
    };
    eframe::run_native("Job Runner", opts, Box::new(|_cc| Ok(Box::new(App::new(conn)))))
        .map_err(|e| anyhow::anyhow!("{e}"))
}
