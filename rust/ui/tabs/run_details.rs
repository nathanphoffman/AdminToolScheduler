//! The right-hand side of the History tab: the chosen run's header, its note, and its console output.

use super::style::{status_color, status_label, AMBER, GREY, RED, STATUS_FAILED, STATUS_SKIPPED, TITLE_SIZE};
use super::App;
use crate::Run;
use eframe::egui::{self, RichText};

pub fn draw_run_details(state: &App, screen: &mut egui::Ui) {
    let Some(past_run) = state.view.selected_run.and_then(|run_id| state.snapshot.run_by_id(run_id)) else {
        screen.label("Pick a run on the left to see its output.");

        return;
    };

    details_header(screen, past_run);
    details_note(screen, past_run);
    screen.separator();
    output_panel(state, screen, past_run);
}

fn details_header(screen: &mut egui::Ui, past_run: &Run) {
    screen.horizontal(|header| {
        header.label(RichText::new(&past_run.job_name).strong().size(TITLE_SIZE));
        header.label(RichText::new(status_label(&past_run.status)).color(status_color(&past_run.status)).strong());

        if past_run.exit_code >= 0 {
            header.label(format!("exit code {}", past_run.exit_code));
        }

        header.label(RichText::new(format!("started by {}", past_run.trigger_kind)).color(GREY));
    });
}

/// The reason a run failed or was skipped, in red for a failure and amber otherwise.
fn details_note(screen: &mut egui::Ui, past_run: &Run) {
    if !past_run.note.is_empty() {
        let color = if past_run.status == STATUS_FAILED { RED } else { AMBER };
        screen.label(RichText::new(&past_run.note).color(color));
    }
}

fn output_panel(state: &App, screen: &mut egui::Ui, past_run: &Run) {
    egui::ScrollArea::both().stick_to_bottom(true).auto_shrink([false, false]).show(screen, |output| {
        if past_run.status == STATUS_SKIPPED {
            output.label("Nothing ran, so there is no output.");
        } else {
            output.add(egui::Label::new(RichText::new(&state.view.log_text).monospace()).selectable(true));
        }
    });
}
