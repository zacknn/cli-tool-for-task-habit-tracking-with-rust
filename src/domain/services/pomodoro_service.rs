use chrono::{DateTime, Utc, Duration};
use crate::domain::models::{PomodoroSession, Task, CompletedSession};
use crate::storage::json_store::JsonStorage;

pub struct PomodoroService {
    storage: JsonStorage,
}

impl PomodoroService {
    pub fn new(storage: JsonStorage) -> Self {
        Self { storage }
    }
    
    // Start a new pomodoro session
    pub fn start_session(
        &self,
        work_duration: u32,
        short_break: u32,
        long_break: u32,
        total_sessions: u32,
        sessions_before_long_break: u32,
        task: Option<Task>,
    ) -> Result<PomodoroSession, String> {
        // Create new session
        let session = PomodoroSession {
            id: 1, // Simple ID for now
            work_duration,
            short_break,
            long_break,
            total_sessions,
            sessions_before_long_break,
            current_session: 1,
            is_break: false,
            task_id: task.as_ref().map(|t| t.id),
            started_at: Utc::now(),
            completed_sessions: Vec::new(),
        };
        
        // Save task if provided
        if let Some(task) = task {
            let mut tasks = self.storage.load_tasks()?;
            tasks.push(task);
            self.storage.save_tasks(&tasks)?;
        }
        
        // Save session
        self.storage.save_current_session(&session)?;
        
        Ok(session)
    }
    
    // Get current session status
    pub fn get_status(&self) -> Result<Option<PomodoroSession>, String> {
        self.storage.load_current_session()
    }
    
    // Complete current work session
    pub fn complete_work_session(&self) -> Result<PomodoroSession, String> {
        let mut session = match self.storage.load_current_session()? {
            Some(s) => s,
            None => return Err("No active pomodoro session".to_string()),
        };
        
        // Record completed work session
        let completed = CompletedSession {
            session_number: session.current_session,
            start_time: Utc::now() - Duration::minutes(session.work_duration as i64),
            end_time: Utc::now(),
            was_break: false,
        };
        
        session.completed_sessions.push(completed);
        
        // Check if we need a long break or short break
        if session.current_session % session.sessions_before_long_break == 0 {
            // Time for long break
            session.is_break = true;
        } else {
            // Time for short break
            session.is_break = true;
        }
        
        self.storage.save_current_session(&session)?;
        Ok(session)
    }
    
    // Complete current break
    pub fn complete_break(&self) -> Result<PomodoroSession, String> {
        let mut session = match self.storage.load_current_session()? {
            Some(s) => s,
            None => return Err("No active pomodoro session".to_string()),
        };
        
        // Record completed break
        let break_duration = if session.current_session % session.sessions_before_long_break == 0 {
            session.long_break
        } else {
            session.short_break
        };
        
        let completed = CompletedSession {
            session_number: session.current_session,
            start_time: Utc::now() - Duration::minutes(break_duration as i64),
            end_time: Utc::now(),
            was_break: true,
        };
        
        session.completed_sessions.push(completed);
        session.is_break = false;
        session.current_session += 1;
        
        // Check if all sessions are complete
        if session.current_session > session.total_sessions {
            // Session is complete
            self.storage.save_current_session(&session)?;
            return Ok(session);
        }
        
        self.storage.save_current_session(&session)?;
        Ok(session)
    }
    
    // Stop current session
    pub fn stop_session(&self) -> Result<(), String> {
        // Remove current session file
        let path = format!("{}/current_session.json", self.storage.data_dir);
        if std::path::Path::new(&path).exists() {
            std::fs::remove_file(&path)
                .map_err(|e| format!("Failed to remove session file: {}", e))?;
        }
        Ok(())
    }
    
    // List all tasks
    pub fn list_tasks(&self) -> Result<Vec<Task>, String> {
        self.storage.load_tasks()
    }
}