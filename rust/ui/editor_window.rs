//! The pop-up window for adding, changing and deleting a job.

use super::editor::{Editor, DAILY, INTERVAL};
use super::style::{GREY, RED, SPACE_MEDIUM};
use super::App;
use crate::Job;
use eframe::egui::{self, RichText};

const WINDOW_WIDTH: f32 = 480.0;
const GRID_SPACING: [f32; 2] = [10.0, 8.0];
const COMMAND_ROWS: usize = 3;
const COMMAND_WIDTH: f32 = 380.0;
const CLOCK_BOX_WIDTH: f32 = 30.0;
const NUMBER_BOX_WIDTH: f32 = 50.0;
const COMMAND_HINT: &str = "Several lines run one after another and stop at the first one that fails.";

enum Action {
    Keep,
    Save,
    Delete,
    Cancel,
}

/// Draws the editor if one is open, then does whatever button was pressed.
pub fn show(state: &mut App, context: &egui::Context) {
    // The job is taken out while it is edited so the form can change it freely.
    let Some(mut target_job) = state.editor.editing.take() else { return };

    let title = if target_job.job_id == 0 { "New job" } else { "Edit job" };
    let mut action = Action::Keep;

    egui::Window::new(title).collapsible(false).default_width(WINDOW_WIDTH).show(context, |screen| {
        form_grid(screen, &mut target_job, &mut state.editor);
        screen.label(RichText::new(state.editor.summary(&target_job)).strong());
        screen.label(RichText::new(COMMAND_HINT).small().color(GREY));

        if let Some(message) = &state.editor.error {
            screen.colored_label(RED, message);
        }

        screen.add_space(SPACE_MEDIUM);
        action = button_row(screen, target_job.job_id);
    });

    apply(state, target_job, action);
}

fn form_grid(screen: &mut egui::Ui, target_job: &mut Job, editor: &mut Editor) {
    // A two-column form: a label on the left, its input on the right.
    egui::Grid::new("edit_grid").num_columns(2).spacing(GRID_SPACING).show(screen, |grid| {
        grid.label("Name");
        grid.text_edit_singleline(&mut target_job.name);
        grid.end_row();

        grid.label("Folder");
        grid.text_edit_singleline(&mut target_job.folder);
        grid.end_row();

        grid.label("Command");
        grid.add(egui::TextEdit::multiline(&mut target_job.command).desired_rows(COMMAND_ROWS).code_editor().desired_width(COMMAND_WIDTH));
        grid.end_row();

        grid.label("Schedule");

        // The two schedule kinds sit side by side as a pair of choices.
        grid.horizontal(|kinds| {
            kinds.selectable_value(&mut target_job.sched_kind, DAILY.to_string(), "Once a day at");
            kinds.selectable_value(&mut target_job.sched_kind, INTERVAL.to_string(), "Every");
        });

        grid.end_row();

        grid.label("");
        schedule_controls(grid, &target_job.sched_kind, editor);
        grid.end_row();

        grid.label("");
        grid.checkbox(&mut target_job.enabled, "Enabled");
        grid.end_row();
    });
}

/// The boxes for the chosen schedule kind: a time of day, or a number with minutes/hours buttons.
fn schedule_controls(screen: &mut egui::Ui, kind: &str, editor: &mut Editor) {
    screen.horizontal(|controls| {
        if kind == DAILY {
            controls.add(egui::TextEdit::singleline(&mut editor.hour).desired_width(CLOCK_BOX_WIDTH));
            controls.label(":");
            controls.add(egui::TextEdit::singleline(&mut editor.minute).desired_width(CLOCK_BOX_WIDTH));
            controls.label("(24 hour clock)");
        } else {
            controls.add(egui::TextEdit::singleline(&mut editor.number).desired_width(NUMBER_BOX_WIDTH));
            controls.selectable_value(&mut editor.in_hours, false, "minutes");
            controls.selectable_value(&mut editor.in_hours, true, "hours");
        }
    });
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

// Carries out the pressed button; with no button pressed the job stays open for more editing.
fn apply(state: &mut App, target_job: Job, action: Action) {
    match action {
        Action::Keep => state.editor.editing = Some(target_job),
        Action::Cancel => state.editor.error = None,
        Action::Save => save(state, target_job),
        Action::Delete => {
            crate::db_delete_job(state.conn.clone(), target_job.job_id);
            state.editor.error = None;
            state.snapshot.mark_stale();
        }
    }
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
