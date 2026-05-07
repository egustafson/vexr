use crate::reminders::{Recurrence, Reminder};
use chrono::{DateTime, Utc};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

const STORE_FILE: &str = "reminders.json";
const STORE_KEY: &str = "reminders";

fn load_reminders(app: &AppHandle) -> Vec<Reminder> {
    let store = app.store(STORE_FILE).expect("failed to open store");
    store
        .get(STORE_KEY)
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default()
}

fn save_reminders(app: &AppHandle, reminders: &[Reminder]) {
    let store = app.store(STORE_FILE).expect("failed to open store");
    store.set(STORE_KEY, serde_json::to_value(reminders).unwrap());
    let _ = store.save();
}

#[tauri::command]
pub fn create_reminder(
    app: AppHandle,
    title: String,
    description: Option<String>,
    trigger_at: DateTime<Utc>,
    recurrence: Recurrence,
) -> Result<Reminder, String> {
    let reminder = Reminder::new(title, description, trigger_at, recurrence);
    let mut reminders = load_reminders(&app);
    reminders.push(reminder.clone());
    save_reminders(&app, &reminders);
    Ok(reminder)
}

#[tauri::command]
pub fn list_reminders(app: AppHandle) -> Vec<Reminder> {
    load_reminders(&app)
}

#[tauri::command]
pub fn update_reminder(
    app: AppHandle,
    id: String,
    title: String,
    description: Option<String>,
    trigger_at: DateTime<Utc>,
    recurrence: Recurrence,
) -> Result<(), String> {
    let mut reminders = load_reminders(&app);
    let r = reminders
        .iter_mut()
        .find(|r| r.id == id)
        .ok_or_else(|| "Reminder not found".to_string())?;
    r.title = title;
    r.description = description;
    r.trigger_at = trigger_at;
    r.next_trigger = trigger_at;
    r.recurrence = recurrence;
    r.fired = false;
    save_reminders(&app, &reminders);
    Ok(())
}

#[tauri::command]
pub fn delete_reminder(app: AppHandle, id: String) -> Result<(), String> {
    let mut reminders = load_reminders(&app);
    let before = reminders.len();
    reminders.retain(|r| r.id != id);
    if reminders.len() == before {
        return Err("Reminder not found".to_string());
    }
    save_reminders(&app, &reminders);
    Ok(())
}

#[tauri::command]
pub fn acknowledge_reminder(app: AppHandle, id: String) -> Result<(), String> {
    let mut reminders = load_reminders(&app);
    let pos = reminders
        .iter()
        .position(|r| r.id == id)
        .ok_or_else(|| "Reminder not found".to_string())?;
    let keep = reminders[pos].acknowledge();
    if !keep {
        reminders.remove(pos);
    }
    save_reminders(&app, &reminders);
    Ok(())
}

/// Called by the scheduler — marks a reminder as fired so it won't re-fire on the next tick.
pub fn mark_fired(app: &AppHandle, id: &str) {
    let mut reminders = load_reminders(app);
    if let Some(r) = reminders.iter_mut().find(|r| r.id == id) {
        r.fired = true;
    }
    save_reminders(app, &reminders);
}
