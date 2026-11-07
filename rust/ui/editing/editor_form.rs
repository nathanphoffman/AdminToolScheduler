//! The fields of the job editor: name, folder, command, schedule and the enabled box.

use super::editor::{Editor, DAILY, INTERVAL};
use crate::Job;
use eframe::egui;

const GRID_SPACING: [f32; 2] = [10.0, 8.0];
const COMMAND_ROWS: usize = 3;
const COMMAND_WIDTH: f32 = 380.0;
const CLOCK_BOX_WIDTH: f32 = 30.0;
const NUMBER_BOX_WIDTH: f32 = 50.0;

pub fn form_grid(screen: &mut egui::Ui, target_job: &mut Job, editor: &mut Editor) {
    // A two-column form: a label on the left, its input on the right.
    egui::Grid::new("edit_grid").num_columns(2).spacing(GRID_SPACING).show(screen, |grid| {
        text_rows(grid, target_job);
        command_row(grid, target_job);
        schedule_rows(grid, target_job, editor);
        enabled_row(grid, target_job);
    });
}

fn text_rows(grid: &mut egui::Ui, target_job: &mut Job) {
    grid.label("Name");
    grid.text_edit_singleline(&mut target_job.name);
    grid.end_row();

    grid.label("Folder");
    grid.text_edit_singleline(&mut target_job.folder);
    grid.end_row();
}

fn command_row(grid: &mut egui::Ui, target_job: &mut Job) {
    grid.label("Command");
    grid.add(egui::TextEdit::multiline(&mut target_job.command).desired_rows(COMMAND_ROWS).code_editor().desired_width(COMMAND_WIDTH));
    grid.end_row();
}

fn schedule_rows(grid: &mut egui::Ui, target_job: &mut Job, editor: &mut Editor) {
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

fn enabled_row(grid: &mut egui::Ui, target_job: &mut Job) {
    grid.label("");
    grid.checkbox(&mut target_job.enabled, "Enabled");
    grid.end_row();
}
