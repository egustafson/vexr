# vexr

> **Vexr: proactively annoying for your own good.**

vexr is a system-tray reminder app built with [Tauri v2](https://tauri.app/) and Rust. It lives quietly in your tray and interrupts you with acknowledgment-required alerts when reminders fire.

## Features

- One-time and recurring reminders (daily, weekly, monthly)
- Tray-based UI — no persistent window
- Alerts that require explicit acknowledgment before dismissal
- Local JSON storage — no cloud, no accounts

## Tech Stack

- **Backend**: Rust, Tauri v2
- **Frontend**: Vanilla JS / HTML / CSS
- **Storage**: `tauri-plugin-store` (JSON)
- **Platforms**: Linux/GNOME, Windows, macOS

## Prerequisites

### Linux
```sh
sudo apt-get install -y libwebkit2gtk-4.1-dev libgtk-3-dev \
  libayatana-appindicator3-dev libxdo-dev build-essential \
  pkg-config libssl-dev
```

### All platforms
- [Rust](https://rustup.rs/) (stable)
- `cargo install tauri-cli --version "^2"`

## Development

```sh
cargo tauri dev
```

## Build

```sh
cargo tauri build
```

## Usage

1. Launch vexr — it appears only in the system tray.
2. Right-click the tray icon → **Set Reminder** to create a reminder.
3. Right-click → **Manage Reminders** to view, edit, or delete reminders.
4. When a reminder fires, an alert window appears on screen. Click **Acknowledge** to dismiss it.
