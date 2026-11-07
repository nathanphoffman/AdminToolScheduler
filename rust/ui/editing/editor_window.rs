//! The pop-up window for adding, changing and deleting a job.

use super::editor::Editor;
use super::editor_form::form_grid;
use super::style::{GREY, RED, SPACE_MEDIUM};
use super::App;
use crate::Job;
use eframe::egui::{self, RichText};

const WINDOW_WIDTH: f32 = 480.0;
const COMMAND_HINT: &str = "Several lines run one after another and stop at the first one that fails.";

enum Action {
    Keep,
    Save,
    Delete,
    Cancel,
}

/// Draws the editor if one is open, then does whatever button was pressed.
pub fn draw_editor(state: &mut App, context: &egui::Context) {
    // The job is taken out while it is edited so the form can change it freely.
    let Some(mut target_job) = state.editor.editing.take() else { return };

    let title = if target_job.job_id == 0 { "New job" } else { "Edit job" };
    let mut action = Action::Keep;

    egui::Window::new(title).collapsible(false).default_width(WINDOW_WIDTH).show(context, |screen| {
        form_grid(screen, &mut target_job, &mut state.editor);
        action = form_footer(&state.editor, screen, &target_job);
    });

    // With no button pressed the job stays open for more editing.
    match action {
        Action::Keep => state.editor.editing = Some(target_job),
        Action::Cancel => state.editor.error = None,
        Action::Save => save(state, target_job),
        Action::Delete => delete(state, target_job),
    }
}

/// The summary of the schedule, the hint, any error, and the buttons; returns the button pressed.
fn form_footer(editor: &Editor, screen: &mut egui::Ui, target_job: &Job) -> Action {
    screen.label(RichText::new(editor.summary(target_job)).strong());
    screen.label(RichText::new(COMMAND_HINT).small().color(GREY));

    if let Some(message) = &editor.error {
        screen.colored_label(RED, message);
    }

    screen.add_space(SPACE_MEDIUM);

    button_row(screen, target_job.job_id)
}

// Save, Cancel and (for an existing job) Delete; reports which one was pressed this frame.
fn button_row(screen: &mut egui::Ui, job_id: i64) -> Action {
    let mut action = Action::Keep;

    screen.horizontal(|buttons| {
        if buttons.button("Save").clicked() {
            action = Action::Save;
        }

        if buttons.button("Cancel").clicked() {
            action = Action::Cancel;
        }

        if job_id != 0 && buttons.button(RichText::new("Delete job").color(RED)).clicked() {
            action = Action::Delete;
        }
    });

    action
}

fn delete(state: &mut App, target_job: Job) {
    crate::db_delete_job(state.conn.clone(), target_job.job_id);
    state.editor.error = None;
    state.snapshot.mark_stale();
}

/// Saves a valid job and closes the editor; an invalid one stays open with the reason shown.
fn save(state: &mut App, target_job: Job) {
    match state.editor.checked_job(&target_job) {
        Ok(checked) => {
            crate::db_save_job(state.conn.clone(), checked);
            state.editor.error = None;
            state.snapshot.mark_stale();
        }

        Err(message) => {
            state.editor.error = Some(message);
            state.editor.editing = Some(target_job);
        }
    }
}
