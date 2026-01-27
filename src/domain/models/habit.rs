use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Habit {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub frequency: String, // e.g., "daily", "weekly"
    pub streak: u32,
    pub created_at: DateTime<Local>,
    pub last_completed_at: Option<DateTime<Local>>,
}
