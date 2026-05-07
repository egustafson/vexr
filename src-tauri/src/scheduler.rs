use crate::commands::mark_fired;
use crate::reminders::Reminder;
use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreExt;
use tokio::time::{interval, Duration};

const STORE_FILE: &str = "reminders.json";
const STORE_KEY: &str = "reminders";
const TICK_SECS: u64 = 30;

pub fn start(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut ticker = interval(Duration::from_secs(TICK_SECS));
        loop {
            ticker.tick().await;
            check_and_fire(&app);
        }
    });
}

fn check_and_fire(app: &AppHandle) {
    let now = chrono::Utc::now();

    let reminders: Vec<Reminder> = {
        let store = match app.store(STORE_FILE) {
            Ok(s) => s,
            Err(_) => return,
        };
        store
            .get(STORE_KEY)
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default()
    };

    for reminder in reminders {
        if !reminder.fired && reminder.next_trigger <= now {
            mark_fired(app, &reminder.id);
            open_alert_window(app, &reminder);
        }
    }
}

fn open_alert_window(app: &AppHandle, reminder: &Reminder) {
    let label = format!("alert-{}", &reminder.id[..8]);
    let id = reminder.id.clone();
    let title = urlencoding_encode(&reminder.title);
    let desc = urlencoding_encode(reminder.description.as_deref().unwrap_or(""));
    let url = format!("alert.html?id={}&title={}&desc={}", id, title, desc);

    if let Some(win) = app.get_webview_window(&label) {
        let _ = win.show();
        let _ = win.set_focus();
        return;
    }

    let _ = tauri::WebviewWindowBuilder::new(
        app,
        label,
        tauri::WebviewUrl::App(url.into()),
    )
    .title("Reminder")
    .inner_size(420.0, 260.0)
    .min_inner_size(420.0, 260.0)
    .max_inner_size(420.0, 260.0)
    .resizable(false)
    .always_on_top(true)
    .closable(false)
    .focused(true)
    .build();
}

fn urlencoding_encode(s: &str) -> String {
    s.bytes()
        .flat_map(|b| {
            if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
                vec![b as char]
            } else {
                format!("%{:02X}", b).chars().collect::<Vec<_>>()
            }
        })
        .collect()
}
