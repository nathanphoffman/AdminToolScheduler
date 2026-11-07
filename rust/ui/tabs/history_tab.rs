//! The History tab: every past run on the left, and the chosen run's result and console output on the right.

use super::run_details;
use super::run_list::run_list;
use super::style::SPACE_SMALL;
use super::App;
use eframe::egui;

const LIST_WIDTH: f32 = 340.0;
const ALL_JOBS: &str = "All jobs";

pub fn draw_history_tab(state: &mut App, screen: &mut egui::Ui) {
    filter_picker(state, screen);
    screen.add_space(SPACE_SMALL);

    egui::Panel::left("runs_list").resizable(true).default_size(LIST_WIDTH).show(screen, |list| run_list(state, list));
    run_details::draw_run_details(state, screen);
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
