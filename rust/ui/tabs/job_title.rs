//! The top line of a job card: enabled box, name, schedule, next run, and the Edit / Run now buttons.

use super::style::{AMBER, GREY, TITLE_SIZE};
use super::App;
use crate::Job;
use eframe::egui::{self, RichText};

pub fn title_row(state: &mut App, screen: &mut egui::Ui, target_job: &Job) {
    // One line: checkbox, name, schedule, next run, then the buttons.
    screen.horizontal(|strip| {
        enabled_checkbox(state, strip, target_job);
        strip.label(RichText::new(&target_job.name).strong().size(TITLE_SIZE));
        strip.label(RichText::new(crate::schedule_text(target_job.clone())).color(GREY));
        next_run_label(state, strip, target_job);

        // Right-to-left layout, so the buttons sit at the card's right edge.
        strip.with_layout(egui::Layout::right_to_left(egui::Align::Center), |buttons| {
            row_buttons(state, buttons, target_job);
        });
    });
}

fn row_buttons(state: &mut App, buttons: &mut egui::Ui, target_job: &Job) {
    if buttons.button("Edit").clicked() {
        state.editor.begin(target_job.clone());
    }

    if buttons.button("Run now").clicked() {
        crate::db_request_run(state.conn.clone(), target_job.job_id);
    }
}

fn enabled_checkbox(state: &mut App, screen: &mut egui::Ui, target_job: &Job) {
    let mut enabled = target_job.enabled;

    if screen.checkbox(&mut enabled, "").on_hover_text("Run this job on its schedule").changed() {
        let mut changed = target_job.clone();
        changed.enabled = enabled;

        // A card always shows a saved job, so it can be updated directly.
        crate::db_update_job(state.conn.clone(), changed);
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
