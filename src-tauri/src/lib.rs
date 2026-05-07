mod commands;
mod reminders;
mod scheduler;
mod tray;

use tauri::Manager;
use commands::{
    acknowledge_reminder, create_reminder, delete_reminder, list_reminders, update_reminder,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            create_reminder,
            list_reminders,
            update_reminder,
            delete_reminder,
            acknowledge_reminder,
        ])
        .setup(|app| {
            tray::setup_tray(app.handle())?;
            scheduler::start(app.handle().clone());

            // Hide the main window — the app lives in the tray.
            if let Some(win) = app.get_webview_window("main") {
                let _ = win.hide();
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running vexr");
}
