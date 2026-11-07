mod daemon;
mod db;
mod tray;
mod ui;

fn main() -> anyhow::Result<()> {
    match std::env::args().nth(1).as_deref() {
        Some("ui") => ui::run(),
        Some("daemon") | None => daemon::run_daemon(),
        Some(other) => anyhow::bail!("Unknown command '{other}'. Use 'daemon' (default) or 'ui'."),
    }
}
