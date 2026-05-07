use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, Runtime,
};

pub fn setup_tray<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let set_reminder = MenuItem::with_id(app, "set-reminder", "Set Reminder", true, None::<&str>)?;
    let manage = MenuItem::with_id(app, "manage-reminders", "Manage Reminders", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&set_reminder, &manage, &separator, &quit])?;

    let icon = app
        .default_window_icon()
        .cloned()
        .expect("no default window icon configured");

    TrayIconBuilder::with_id("main-tray")
        .icon(icon)
        .icon_as_template(true)
        .tooltip("Vexr")
        .menu(&menu)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "set-reminder" => open_window(app, "set-reminder"),
            "manage-reminders" => open_window(app, "manage-reminders"),
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                open_window(tray.app_handle(), "manage-reminders");
            }
        })
        .build(app)?;

    Ok(())
}

fn open_window<R: Runtime>(app: &AppHandle<R>, label: &str) {
    if let Some(win) = app.get_webview_window(label) {
        let _ = win.show();
        let _ = win.set_focus();
    }
}
