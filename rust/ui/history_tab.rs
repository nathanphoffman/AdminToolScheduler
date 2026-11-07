//! The History tab: every past run on the left, and the chosen run's result and console output on the right.

use super::style::{duration_text, status_color, status_label, AMBER, GREY, RED, SPACE_SMALL, TITLE_SIZE};
use super::App;
use crate::Run;
use eframe::egui::{self, RichText};

const LIST_WIDTH: f32 = 340.0;
const ALL_JOBS: &str = "All jobs";

pub fn show(state: &mut App, screen: &mut egui::Ui) {
    filter_picker(state, screen);
    screen.add_space(SPACE_SMALL);

    egui::Panel::left("runs_list").resizable(true).default_size(LIST_WIDTH).show(screen, |list| run_list(state, list));
    run_details(state, screen);
}

fn filter_name(state: &App) -> String {
    for target_job in &state.snapshot.jobs {
        if Some(target_job.job_id) == state.view.filter_job {
            return target_job.name.clone();
        }
    }

    ALL_JOBS.to_string()
}

fn filter_picker(state: &mut App, screen: &mut egui::Ui) {
    let mut new_filter = state.view.filter_job;
    let current_name = filter_name(state);

    // The label and the drop-down sit side by side.
    screen.horizontal(|strip| {
        strip.label("Job:");

        // The drop-down offers 'All jobs' then each job; picking one sets new_filter.
        egui::ComboBox::from_id_salt("filter").selected_text(current_name).show_ui(strip, |choices| {
            choices.selectable_value(&mut new_filter, None, ALL_JOBS);

            for target_job in &state.snapshot.jobs {
                choices.selectable_value(&mut new_filter, Some(target_job.job_id), &target_job.name);
            }
        });
    });

    if new_filter != state.view.filter_job {
        state.view.filter_job = new_filter;
        state.snapshot.mark_stale();
    }
}

fn run_list(state: &mut App, screen: &mut egui::Ui) {
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
    overlay.horizontal(|line| {
        line.label(RichText::new(status_label(&past_run.status)).color(status_color(&past_run.status)).strong());
        line.label(&past_run.job_name);
    });

    let click = screen.interact(row_area, screen.make_persistent_id(("run", past_run.run_id)), egui::Sense::click());
    let when = format!("{}  ({})", crate::time_format(past_run.started_at), duration_text(past_run));
    screen.label(RichText::new(when).small().color(GREY));

    if click.clicked() {
        state.view.select_run(past_run.run_id);
        state.snapshot.mark_stale();
    }
}

// The right-hand side: the chosen run's header, its failure note, and its console output.
fn run_details(state: &App, screen: &mut egui::Ui) {
    let Some(past_run) = state.view.selected_run.and_then(|run_id| state.snapshot.run_by_id(run_id)) else {
        screen.label("Pick a run on the left to see its output.");

        return;
    };

    screen.horizontal(|header| {
        header.label(RichText::new(&past_run.job_name).strong().size(TITLE_SIZE));
        header.label(RichText::new(status_label(&past_run.status)).color(status_color(&past_run.status)).strong());

        if past_run.exit_code >= 0 {
            header.label(format!("exit code {}", past_run.exit_code));
        }

        header.label(RichText::new(format!("started by {}", past_run.trigger_kind)).color(GREY));
    });

    if !past_run.note.is_empty() {
        let color = if past_run.status == "failed" { RED } else { AMBER };
        screen.label(RichText::new(&past_run.note).color(color));
    }

    screen.separator();

    egui::ScrollArea::both().stick_to_bottom(true).auto_shrink([false, false]).show(screen, |output| {
        if past_run.status == "skipped" {
            output.label("Nothing ran, so there is no output.");
        } else {
            output.add(egui::Label::new(RichText::new(&state.view.log_text).monospace()).selectable(true));
        }
    });
}
