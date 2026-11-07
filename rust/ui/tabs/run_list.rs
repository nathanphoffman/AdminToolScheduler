//! The left-hand side of the History tab: one row per past run, newest first.

use super::style::{duration_text, status_color, status_label, GREY};
use super::App;
use crate::Run;
use eframe::egui::{self, RichText};

pub fn run_list(state: &mut App, screen: &mut egui::Ui) {
    // Copied so clicking a row can change the app while the rows are being drawn.
    let runs = state.snapshot.runs.clone();

    egui::ScrollArea::vertical().show(screen, |rows| {
        for past_run in &runs {
            run_row(state, rows, past_run);
            rows.separator();
        }
    });
}

fn run_row(state: &mut App, screen: &mut egui::Ui, past_run: &Run) {
    let selected = state.view.selected_run == Some(past_run.run_id);
    let row_area = screen.selectable_label(selected, "").rect;

    // The label above only reserves the row and its highlight; the coloured text is drawn on top of it.
    let mut overlay = screen.new_child(egui::UiBuilder::new().max_rect(row_area));
    row_text(&mut overlay, past_run);

    let click = screen.interact(row_area, screen.make_persistent_id(("run", past_run.run_id)), egui::Sense::click());
    let when = format!("{}  ({})", crate::time_format(past_run.started_at), duration_text(past_run));
    screen.label(RichText::new(when).small().color(GREY));

    if click.clicked() {
        state.view.select_run(past_run.run_id);
        state.snapshot.mark_stale();
    }
}

fn row_text(screen: &mut egui::Ui, past_run: &Run) {
    screen.horizontal(|line| {
        line.label(RichText::new(status_label(&past_run.status)).color(status_color(&past_run.status)).strong());
        line.label(&past_run.job_name);
    });
}
