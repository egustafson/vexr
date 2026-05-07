use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Recurrence {
    Once,
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reminder {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub trigger_at: DateTime<Utc>,
    pub recurrence: Recurrence,
    pub next_trigger: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub fired: bool,
}

impl Reminder {
    pub fn new(
        title: String,
        description: Option<String>,
        trigger_at: DateTime<Utc>,
        recurrence: Recurrence,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            next_trigger: trigger_at,
            trigger_at,
            recurrence,
            created_at: now,
            fired: false,
        }
    }

    /// Advance next_trigger for recurring reminders after acknowledgment.
    /// Returns true if the reminder should be kept, false if it should be removed.
    pub fn acknowledge(&mut self) -> bool {
        use chrono::Months;
        use std::time::Duration as StdDuration;

        match self.recurrence {
            Recurrence::Once => false,
            Recurrence::Daily => {
                self.next_trigger = self.next_trigger
                    + chrono::Duration::from_std(StdDuration::from_secs(86400)).unwrap();
                self.fired = false;
                true
            }
            Recurrence::Weekly => {
                self.next_trigger = self.next_trigger
                    + chrono::Duration::from_std(StdDuration::from_secs(7 * 86400)).unwrap();
                self.fired = false;
                true
            }
            Recurrence::Monthly => {
                self.next_trigger = self.next_trigger + Months::new(1);
                self.fired = false;
                true
            }
        }
    }
}
