//! The tray dot: its three states and how each one is drawn.

use ksni::Icon;

const ICON_SIZE: i32 = 24;
const CIRCLE_RADIUS: f32 = 10.0;
const EDGE_WIDTH: f32 = 1.0;
const SOLID_ALPHA: u8 = 255;
const EDGE_ALPHA: u8 = 120;
const RUNNING_GREEN: (u8, u8, u8) = (46, 160, 67);
const FAILED_RED: (u8, u8, u8) = (215, 58, 73);
const IDLE_BLUE: (u8, u8, u8) = (88, 130, 200);

#[derive(Clone, Copy, PartialEq)]
pub enum TrayStatus {
    Idle,
    Running,
    Failed,
}

impl TrayStatus {
    /// Anything unrecognised counts as idle, so a typo upstream shows a calm icon rather than a wrong alarm.
    pub fn from_name(name: &str) -> TrayStatus {
        match name {
            "running" => TrayStatus::Running,
            "failed" => TrayStatus::Failed,
            _ => TrayStatus::Idle,
        }
    }
}

pub fn icon_for(status: TrayStatus) -> Icon {
    let color = match status {
        TrayStatus::Running => RUNNING_GREEN,
        TrayStatus::Failed => FAILED_RED,
        TrayStatus::Idle => IDLE_BLUE,
    };

    let middle = (ICON_SIZE as f32 - 1.0) / 2.0;
    let mut data = Vec::with_capacity((ICON_SIZE * ICON_SIZE * 4) as usize);

    for pixel_row in 0..ICON_SIZE {
        for column in 0..ICON_SIZE {
            let distance = ((column as f32 - middle).powi(2) + (pixel_row as f32 - middle).powi(2)).sqrt();
            let alpha = if distance <= CIRCLE_RADIUS {
                SOLID_ALPHA
            } else if distance <= CIRCLE_RADIUS + EDGE_WIDTH {
                EDGE_ALPHA
            } else {
                0
            };

            // The tray wants ARGB in network byte order.
            data.extend_from_slice(&[alpha, color.0, color.1, color.2]);
        }
    }

    Icon { width: ICON_SIZE, height: ICON_SIZE, data }
}
