# Turn the Deor source into Rust. Edit the .deor files, never build/main.rs.
transpile:
    mkdir -p build
    DEOR_LIB=lib deor main.deor build/main.rs

# Transpile, then compile the release binary at target/release/jobrunner.
build: transpile
    cargo build --release

# Run the background service (tray icon + scheduler) in this terminal.
daemon: build
    ./target/release/jobrunner daemon

# Open just the window.
ui: build
    ./target/release/jobrunner ui
