//! What the desktop's tray asks for: a title, a coloured dot, a tooltip, a click action and a menu.

use super::icon::{icon_for, TrayStatus};
use super::window::open_window;
use ksni::{Icon, ToolTip};

pub struct JobTray {
    pub status: TrayStatus,
}

impl ksni::Tray for JobTray {
    fn id(&self) -> String {
        "jobrunner".into()
    }

    fn title(&self) -> String {
        crate::APP_NAME.into()
    }

    fn icon_pixmap(&self) -> Vec<Icon> {
        vec![icon_for(self.status)]
    }

    // The hover text mirrors the dot's colour, for anyone who cannot tell the colours apart.
    fn tool_tip(&self) -> ToolTip {
        let description = match self.status {
            TrayStatus::Running => "A job is running",
            TrayStatus::Failed => "Idle. The last run failed",
            TrayStatus::Idle => "Idle",
        };

        ToolTip { title: crate::APP_NAME.into(), description: description.into(), ..Default::default() }
    }

    fn activate(&mut self, _horizontal: i32, _vertical: i32) {
        open_window();
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;

        vec![
            StandardItem { label: "Open Job Runner".into(), activate: Box::new(|_| open_window()), ..Default::default() }.into(),
            MenuItem::Separator,
            StandardItem {
                label: "Quit".into(),
                icon_name: "application-exit".into(),
                activate: Box::new(|_| std::process::exit(0)),
                ..Default::default()
            }
            .into(),
        ]
    }
}
