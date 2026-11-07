use ksni::blocking::{Handle, TrayMethods};
use ksni::{Icon, ToolTip};
use std::process::{Child, Command};
use std::sync::Mutex;

static UI_CHILD: Mutex<Option<Child>> = Mutex::new(None);

/// Opens the window (a separate process) unless it is already open.
pub fn open_window() {
    let mut guard = UI_CHILD.lock().unwrap();
    if let Some(child) = guard.as_mut() {
        if matches!(child.try_wait(), Ok(None)) {
            return;
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        *guard = Command::new(exe).arg("ui").spawn().ok();
    }
}

struct JobTray {
    running: usize,
    last_failed: bool,
}

fn circle(rgb: (u8, u8, u8)) -> Icon {
    let size = 24i32;
    let mut data = Vec::with_capacity((size * size * 4) as usize);
    let c = (size as f32 - 1.0) / 2.0;
    for y in 0..size {
        for x in 0..size {
            let d = (((x as f32 - c).powi(2) + (y as f32 - c).powi(2)) as f32).sqrt();
            let a = if d <= 10.0 { 255 } else if d <= 11.0 { 120 } else { 0 };
            // ARGB, network byte order
            data.extend_from_slice(&[a, rgb.0, rgb.1, rgb.2]);
        }
    }
    Icon { width: size, height: size, data }
}

impl ksni::Tray for JobTray {
    fn id(&self) -> String {
        "jobrunner".into()
    }
    fn title(&self) -> String {
        "Job Runner".into()
    }
    fn icon_pixmap(&self) -> Vec<Icon> {
        let color = if self.running > 0 {
            (46, 160, 67) // green: something is running
        } else if self.last_failed {
            (215, 58, 73) // red: last run failed
        } else {
            (88, 130, 200) // blue: idle
        };
        vec![circle(color)]
    }
    fn tool_tip(&self) -> ToolTip {
        let text = if self.running > 0 {
            format!("{} job(s) running", self.running)
        } else if self.last_failed {
            "Idle. The last run failed".to_string()
        } else {
            "Idle".to_string()
        };
        ToolTip { title: "Job Runner".into(), description: text, ..Default::default() }
    }
    fn activate(&mut self, _x: i32, _y: i32) {
        open_window();
    }
    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;
        vec![
            StandardItem {
                label: "Open Job Runner".into(),
                activate: Box::new(|_| open_window()),
                ..Default::default()
            }
            .into(),
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

static TRAY_HANDLE: Mutex<Option<Handle<JobTray>>> = Mutex::new(None);

/// Puts the icon in the system tray. Without a tray the service still runs, just without an icon.
pub fn start() {
    match (JobTray { running: 0, last_failed: false }).spawn() {
        Ok(handle) => *TRAY_HANDLE.lock().unwrap() = Some(handle),
        Err(error) => eprintln!("Could not create the tray icon: {error}"),
    }
}

pub fn update(running: usize, last_failed: bool) {
    if let Some(handle) = TRAY_HANDLE.lock().unwrap().as_ref() {
        handle.update(|tray| {
            tray.running = running;
            tray.last_failed = last_failed;
        });
    }
}
