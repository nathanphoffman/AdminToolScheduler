//! The Jobs tab: one card per job with its schedule, next run, last result and buttons.

use super::style::{duration_text, status_color, status_label, AMBER, GREY, RED, SPACE_MEDIUM, TITLE_SIZE};
use super::App;
use crate::Job;
use eframe::egui::{self, RichText};

pub fn show(state: &mut App, screen: &mut egui::Ui) {
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

fn title_row(state: &mut App, screen: &mut egui::Ui, target_job: &Job) {
    // One line: checkbox, name, schedule, next run, then the buttons.
    screen.horizontal(|strip| {
        enabled_checkbox(state, strip, target_job);
        strip.label(RichText::new(&target_job.name).strong().size(TITLE_SIZE));
        strip.label(RichText::new(crate::schedule_text(target_job.clone())).color(GREY));
        next_run_label(state, strip, target_job);

        // Right-to-left layout, so the buttons sit at the card's right edge.
        strip.with_layout(egui::Layout::right_to_left(egui::Align::Center), |buttons| {
            if buttons.button("Edit").clicked() {
                state.editor.begin(target_job.clone());
            }

            if buttons.button("Run now").clicked() {
                crate::db_request_run(state.conn.clone(), target_job.job_id);
            }
        });
    });
}

fn enabled_checkbox(state: &mut App, screen: &mut egui::Ui, target_job: &Job) {
    let mut enabled = target_job.enabled;

    if screen.checkbox(&mut enabled, "").on_hover_text("Run this job on its schedule").changed() {
        let mut changed = target_job.clone();
        changed.enabled = enabled;
        crate::db_save_job(state.conn.clone(), changed);
        state.snapshot.mark_stale();
    }
}

fn next_run_label(state: &App, screen: &mut egui::Ui, target_job: &Job) {
    if !target_job.enabled {
        screen.label(RichText::new("(off)").color(AMBER));

        return;
    }

    // The service writes each job's next run time; 0 means it has not published one yet.
    let next_run = crate::db_get_meta(state.conn.clone(), format!("next_{}", target_job.job_id));

    if next_run > 0 {
        screen.label(RichText::new(format!("next run {}", crate::time_format(next_run))).color(GREY));
    }
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

    if past_run.status == "failed" && !past_run.note.is_empty() {
        screen.label(RichText::new(&past_run.note).color(RED));
    }
}
