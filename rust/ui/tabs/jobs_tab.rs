//! The Jobs tab: one card per job with its schedule, next run, last result and buttons.

use super::style::{duration_text, status_color, status_label, GREY, RED, SPACE_MEDIUM, STATUS_FAILED};
use super::job_title::title_row;
use super::App;
use crate::Job;
use eframe::egui::{self, RichText};

pub fn draw_jobs_tab(state: &mut App, screen: &mut egui::Ui) {
    if screen.button("+ New job").clicked() {
        state.editor.begin(crate::blank_job());
    }

    screen.add_space(SPACE_MEDIUM);

    // Copied so the cards can change the app (save, refresh) while the list is being drawn.
    let jobs = state.snapshot.jobs.clone();
    egui::ScrollArea::vertical().show(screen, |list| {
        for target_job in &jobs {
            job_card(state, list, target_job);
        }
    });
}

fn job_card(state: &mut App, screen: &mut egui::Ui, target_job: &Job) {
    screen.group(|card| {
        card.set_width(card.available_width());
        title_row(state, card, target_job);
        status_row(state, card, target_job);
    });
}

// The last result line of a job card, with a link that opens that run's log.
fn status_row(state: &mut App, screen: &mut egui::Ui, target_job: &Job) {
    let Some(past_run) = state.snapshot.latest_run_of(target_job.job_id).cloned() else {
        screen.label(RichText::new("Never run").color(GREY));

        return;
    };

    screen.horizontal(|strip| {
        strip.label(RichText::new(status_label(&past_run.status)).color(status_color(&past_run.status)).strong());
        strip.label(format!("{} ({})", crate::time_format(past_run.started_at), duration_text(&past_run)));

        if strip.link("view log").clicked() {
            state.open_log(past_run.run_id);
        }
    });

    if past_run.status == STATUS_FAILED && !past_run.note.is_empty() {
        screen.label(RichText::new(&past_run.note).color(RED));
    }
}
