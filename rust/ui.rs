//! The window: a Jobs tab, a History tab and a job editor. It reads the same SQLite file as the
//! background service and talks to it only through that file ("Run now" is a row in a queue table).

#[path = "ui/editing/editor.rs"]
mod editor;

#[path = "ui/editing/editor_window.rs"]
mod editor_window;

#[path = "ui/tabs/history_tab.rs"]
mod history_tab;

#[path = "ui/tabs/jobs_tab.rs"]
mod jobs_tab;

#[path = "ui/snapshot.rs"]
mod snapshot;

#[path = "ui/style.rs"]
mod style;

#[path = "ui/top_bar.rs"]
mod top_bar;

#[path = "ui/tabs/run_list.rs"]
mod run_list;

#[path = "ui/tabs/run_details.rs"]
mod run_details;

#[path = "ui/tabs/job_title.rs"]
mod job_title;

#[path = "ui/editing/editor_form.rs"]
mod editor_form;

use crate::Database;
use eframe::egui;
use editor::Editor;
use snapshot::{Snapshot, View};
use std::time::Duration;

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
        // The snapshot starts out stale, so the first frame loads the data.
        App { conn, tab: Tab::Jobs, snapshot: Snapshot::empty(), view: View::empty(), editor: Editor::blank() }
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
}

impl eframe::App for App {
    // Runs every frame: reload if the data is old, draw the tab bar and the chosen tab, then the editor if one is open.
    fn ui(&mut self, screen: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.snapshot.is_stale() {
            self.refresh();
        }

        screen.ctx().request_repaint_after(REPAINT_INTERVAL);

        top_bar::draw_top_bar(self, screen);

        match self.tab {
            Tab::Jobs => jobs_tab::draw_jobs_tab(self, screen),
            Tab::History => history_tab::draw_history_tab(self, screen),
        }

        let context = screen.ctx().clone();
        editor_window::draw_editor(self, &context);
    }
}

pub fn show_window() -> anyhow::Result<()> {
    let conn = crate::db_open();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(WINDOW_SIZE).with_title(crate::APP_NAME),
        ..Default::default()
    };

    eframe::run_native(crate::APP_NAME, options, Box::new(|_creation| Ok(Box::new(App::with_connection(conn)))))
        .map_err(|error| anyhow::anyhow!("{error}"))
}
