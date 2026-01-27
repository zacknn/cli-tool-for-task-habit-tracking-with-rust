use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    /// Unique identifier for the task
    pub id: u32,
    /// Title of the task
    pub title: String,
    /// Optional description of the task
    pub description: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Local>,
    /// Completion status
    pub completed: bool,
    /// Completion timestamp
    pub completed_at: Option<DateTime<Local>>,
}
