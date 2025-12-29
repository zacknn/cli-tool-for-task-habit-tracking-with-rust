use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct PomodoroSession {
    pub id: u32,
    pub work_duration: u32, // in minutes
    pub short_break: u32,
    pub long_break: u32,
    pub total_sessions: u32, // for example 6
    pub sessions_before_long_break: u32, // eg : 4
    pub current_session: u32, // which session we're on (1, 2, 3...)
    pub is_break: bool, // true if currently on a break
    pub task_id: Option<u32>, // optional task associated 
    pub started_at: DateTime<Utc>,
    pub completed_sessions: Vec<CompletedSession>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletedSession {
    pub session_number: u32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub was_break: bool,
}