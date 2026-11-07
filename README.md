# Job Runner

A small background service for Linux that runs your jobs on a schedule, with a tray icon and a window to see what ran.

- **Schedules:** once a day at a set time, or every N minutes/hours. If the computer was asleep or off at the scheduled time, that run is skipped (and recorded as skipped).
- **History:** every run is saved with its result, exit code and full console output. A failed run says why.
- **Tray icon:** blue when idle, green while a job runs, red if the last run failed. Click it to open the window.
- **Window:** turn jobs on or off, edit schedules and commands, press "Run now", and read past logs.

## How it is built

The logic is written in [Deor](https://github.com/nathanphoffman/DeorLang), which turns into Rust. Only the window and tray icon are hand-written Rust, because they need closures, which Deor does not have.

| Folder | What is in it |
|---|---|
| `app/` | Deor source: the job and run records, scheduling rules, the run flow, and the SQLite functions |
| `rust/` | Hand-written Rust: the window (`ui.rs`, `ui/`) and the tray icon (`tray.rs`, `tray/`) |
| `lib/` | Deor's standard library, copied in from the Deor install |
| `build/` | The generated Rust (git-ignored). Edit the `.deor` files, never this |

Runs, jobs and logs are stored in `~/.local/share/jobrunner/`. The window and the service talk only through that SQLite file.

## Build and run

You need [Rust](https://rustup.rs), [just](https://github.com/casey/just) and the `deor` command ([install](https://github.com/nathanphoffman/DeorLang)).

```sh
just build     # Deor -> Rust -> target/release/jobrunner
just daemon    # run the service and tray icon in this terminal
just ui        # open only the window
```

To start it at login, add `~/.config/autostart/jobrunner.desktop` that runs the binary through your shell (so tools like `node` and `just` are found):

```
[Desktop Entry]
Type=Application
Name=Job Runner
Exec=bash -ic '/path/to/target/release/jobrunner daemon'
```

On GNOME the tray icon needs the AppIndicator extension. Cinnamon, KDE and XFCE show it out of the box.

## First start

The first start creates three jobs: Proton Drive sync (daily 02:00), GraphData prod sync (hourly) and Line quality check + sync (daily 01:00). Change or delete them in the window.
