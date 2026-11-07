//! The strip across the top of the window: the two tabs, and whether the background service is alive.

use super::snapshot::Snapshot;
use super::style::{GREEN, RED, SPACE_SMALL};
use super::{App, Tab};
use eframe::egui::{self, RichText};

pub fn draw_top_bar(state: &mut App, screen: &mut egui::Ui) {
    // A strip across the top: tabs on the left, the service status on the right.
    egui::Panel::top("top").show(screen, |bar| {
        bar.add_space(SPACE_SMALL);

        // Everything in the strip sits on one line.
        bar.horizontal(|strip| {
            strip.selectable_value(&mut state.tab, Tab::Jobs, "Jobs");
            strip.selectable_value(&mut state.tab, Tab::History, "History");

            // Right-to-left layout, so the status text hugs the window's right edge.
            strip.with_layout(egui::Layout::right_to_left(egui::Align::Center), |status| {
                service_status(&state.snapshot, status);
            });
        });

        bar.add_space(SPACE_SMALL);
    });
}

fn service_status(snapshot: &Snapshot, screen: &mut egui::Ui) {
    if snapshot.daemon_alive {
        screen.label(RichText::new("● Background service running").color(GREEN));
    } else {
        screen.label(RichText::new("● Background service NOT running - nothing will be scheduled").color(RED));
    }
}
