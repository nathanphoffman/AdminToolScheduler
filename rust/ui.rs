//! The window: a Jobs tab, a History tab and a job editor. It reads the same SQLite file as the
//! background service and talks to it only through that file ("Run now" is a row in a queue table).

#[path = "ui/editor.rs"]
mod editor;

#[path = "ui/editor_window.rs"]
mod editor_window;

#[path = "ui/history_tab.rs"]
mod history_tab;

#[path = "ui/jobs_tab.rs"]
mod jobs_tab;

#[path = "ui/snapshot.rs"]
mod snapshot;

#[path = "ui/style.rs"]
mod style;

use crate::Database;
use eframe::egui::{self, RichText};
use editor::Editor;
use snapshot::{Snapshot, View};
use std::time::Duration;
use style::{GREEN, RED, SPACE_SMALL};

const WINDOW_SIZE: [f32; 2] = [960.0, 620.0];
const REPAINT_INTERVAL: Duration = Duration::from_secs(1);

#[derive(PartialEq)]
enum Tab {
    Jobs,
    History,
}

pub struct App {
    conn: Database,
    tab: Tab,
    snapshot: Snapshot,
    view: View,
    editor: Editor,
}

impl App {
    fn with_connection(conn: Database) -> Self {
        let mut state = App { conn, tab: Tab::Jobs, snapshot: Snapshot::empty(), view: View::empty(), editor: Editor::blank() };
        state.refresh();

        state
    }

    fn refresh(&mut self) {
        self.snapshot.reload(&self.conn, self.view.filter_job);
        self.view.refresh_log(&self.snapshot);
    }

    /// Switches to the History tab with one run's log open.
    fn open_log(&mut self, run_id: i64) {
        self.tab = Tab::History;
        self.view.filter_job = None;
        self.view.select_run(run_id);
        self.snapshot.mark_stale();
    }

    fn top_bar(&mut self, screen: &mut egui::Ui) {
        // A strip across the top: tabs on the left, whether the background service is alive on the right.
        egui::Panel::top("top").show(screen, |bar| {
            bar.add_space(SPACE_SMALL);

            // Everything in the strip sits on one line.
            bar.horizontal(|strip| {
                strip.selectable_value(&mut self.tab, Tab::Jobs, "Jobs");
                strip.selectable_value(&mut self.tab, Tab::History, "History");

                // Right-to-left layout, so the status text hugs the window's right edge.
                strip.with_layout(egui::Layout::right_to_left(egui::Align::Center), |status| {
                    if self.snapshot.daemon_alive {
                        status.label(RichText::new("● Background service running").color(GREEN));
                    } else {
                        status.label(RichText::new("● Background service NOT running - nothing will be scheduled").color(RED));
                    }
                });
            });

            bar.add_space(SPACE_SMALL);
        });
    }
}

impl eframe::App for App {
    // Runs every frame: reload if the data is old, draw the tab bar and the chosen tab, then the editor if one is open.
    fn ui(&mut self, screen: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.snapshot.is_stale() {
            self.refresh();
        }

        screen.ctx().request_repaint_after(REPAINT_INTERVAL);

        self.top_bar(screen);

        match self.tab {
            Tab::Jobs => jobs_tab::show(self, screen),
            Tab::History => history_tab::show(self, screen),
        }

        let context = screen.ctx().clone();
        editor_window::show(self, &context);
    }
}

pub fn show_window() -> anyhow::Result<()> {
    let conn = crate::db_open();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(WINDOW_SIZE).with_title("Job Runner"),
        ..Default::default()
    };

    eframe::run_native("Job Runner", options, Box::new(|_creation| Ok(Box::new(App::with_connection(conn)))))
        .map_err(|error| anyhow::anyhow!("{error}"))
}
