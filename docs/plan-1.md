# Plan: vexr — Tauri Reminder App

## Stack
- Rust + Tauri v2
- Vanilla JS/HTML/CSS (no frontend framework)
- tauri-plugin-store (JSON file storage)
- tauri-plugin-notification (toast)
- MIT License

## Phase 1: Repo & Bootstrap
1. Create .gitignore (Rust + Tauri + OS artifacts)
2. Create LICENSE (MIT)
3. Create README.md (project overview, tagline, build instructions)
4. Scaffold Tauri v2 project: `cargo create-tauri-app` with "vanilla" frontend template
5. Add Cargo dependencies: tauri-plugin-store, tauri-plugin-notification, uuid, chrono, tokio
6. Register plugins in lib.rs and tauri.conf.json
7. Verify cold build succeeds

## Phase 2: Tray & App Shell
1. Source/create tray icon assets (PNG 32x32, ICO for Windows)
2. Configure tray icon in tauri.conf.json
3. Set main window to hidden at startup; app lives in tray
4. Build tray context menu: "Set Reminder" | "Manage Reminders" | separator | "Quit"
5. Wire menu item click handlers to open labeled windows

## Phase 3: Data Layer (Rust)
1. Define Reminder struct: id (UUIDv4), title, description (Option), trigger_at (DateTime<Utc>), recurrence (None/Daily/Weekly/Monthly), next_trigger (DateTime<Utc>), created_at
2. Implement CRUD via tauri-plugin-store in reminders.rs:
   - create_reminder, list_reminders, update_reminder, delete_reminder, acknowledge_reminder
3. On acknowledge of recurring reminder: compute next_trigger and update store
4. Expose all as #[tauri::command] in commands.rs

## Phase 4: Set Reminder Window
1. src/set-reminder.html: form with title (required), description (textarea), date+time picker, recurrence dropdown (Once / Daily / Weekly / Monthly)
2. JS: invoke("create_reminder", {...}) on submit, close window on success
3. Register window in tauri.conf.json (label: "set-reminder", hidden initially, min-size enforced)

## Phase 5: Manage Reminders Window
1. src/manage-reminders.html: fetch and render list via invoke("list_reminders")
2. Each row: title, next trigger time, recurrence badge, Edit button, Delete button
3. Edit: repopulate form fields inline or open set-reminder window pre-filled via window data
4. Delete: confirm then invoke("delete_reminder", {id})
5. Register window in tauri.conf.json (label: "manage-reminders")

## Phase 6: Scheduler & Alert
1. Spawn background Tokio task in lib.rs at startup (interval: check every 30s)
2. On tick: load reminders from store, compare next_trigger <= now()
3. For each due reminder: emit Tauri event "reminder-fired" with reminder payload
4. Listen for event in Rust, open alert window (label: "reminder-alert-{id}"), pass payload via window URL query param or tauri-plugin-window-state
5. src/alert.html: display title + description, single "Acknowledge" button
   - On click: invoke("acknowledge_reminder", {id}), close window
6. Alert window: always-on-top, non-resizable, cannot be closed via OS chrome (intercepted)

## Phase 7: GitHub CI
1. .github/workflows/ci.yml: on push/PR to main — cargo fmt check, cargo clippy, cargo test
2. Optional release workflow using tauri-action for multi-platform builds

## Key Files
- src-tauri/src/lib.rs — plugin registration, scheduler spawn, app setup
- src-tauri/src/reminders.rs — Reminder struct, store CRUD
- src-tauri/src/commands.rs — #[tauri::command] exports
- src-tauri/src/tray.rs — tray icon + context menu setup
- src-tauri/tauri.conf.json — window definitions, tray config, permissions
- src-tauri/Cargo.toml — dependencies
- src/set-reminder.html — create reminder form
- src/manage-reminders.html — list/edit/delete reminders
- src/alert.html — acknowledgment-required alert dialog

## Verification
1. `cargo tauri dev` launches without errors; app appears in system tray
2. Context menu shows all items; clicking opens correct windows
3. Create a one-time reminder 1 minute out; alert window appears and cannot be dismissed without clicking Acknowledge
4. Create a daily reminder; after acknowledge, next_trigger advances by 1 day in store
5. Manage Reminders lists all reminders; edit and delete work
6. CI workflow passes on a test push

## Decisions
- No npm/bundler; Tauri v2 serves src/ as static files directly
- Times stored as UTC ISO-8601 strings in JSON store; displayed in local timezone via JS Intl API
- Each alert opens as its own labeled window to support multiple simultaneous alerts
- Recurrence computation happens Rust-side on acknowledge
- Scope excludes: external alert ingestion (future iteration), cloud sync, snooze functionality
