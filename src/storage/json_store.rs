use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use crate::domain::models::{Task, PomodoroSession};

pub struct JsonStorage {
    pub data_dir: String,
}

impl JsonStorage {
     // Create data directory if it doesnt exist
    pub fn new(data_dir: &str) -> Self {
        if !Path::new(data_dir).exists() {
            fs::create_dir_all(data_dir).expect("Failed to create data directory");
        }
        Self {
            data_dir: data_dir.to_string(),
        }
    }
    // Save tasks
    pub fn save_tasks(&self, tasks: &[Task]) -> Result<(), String> {
        let path = format!("{}/tasks.json", self.data_dir);
        let json = serde_json::to_string_pretty(tasks)
            .map_err(|e| format!("Failed to serialize tasks: {}", e))?;
        
        fs::write(&path, json)
            .map_err(|e| format!("Failed to write tasks to file: {}", e))?;
        
        Ok(())
    }
    
    // Load tasks
    pub fn load_tasks(&self) -> Result<Vec<Task>, String> {
        let path = format!("{}/tasks.json", self.data_dir);
        
        if !Path::new(&path).exists() {
            return Ok(Vec::new());
        }
        
        let json = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read tasks file: {}", e))?;
        
        let tasks = serde_json::from_str(&json)
            .map_err(|e| format!("Failed to parse tasks: {}", e))?;
        
        Ok(tasks)
    }
    
    // Save current pomodoro session
    pub fn save_current_session(&self, session: &PomodoroSession) -> Result<(), String> {
        let path = format!("{}/current_session.json", self.data_dir);
        let json = serde_json::to_string_pretty(session)
            .map_err(|e| format!("Failed to serialize session: {}", e))?;
        
        fs::write(&path, json)
            .map_err(|e| format!("Failed to write session to file: {}", e))?;
        
        Ok(())
    }
    
    // Load current pomodoro session
    pub fn load_current_session(&self) -> Result<Option<PomodoroSession>, String> {
        let path = format!("{}/current_session.json", self.data_dir);
        
        if !Path::new(&path).exists() {
            return Ok(None);
        }
        
        let json = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read session file: {}", e))?;
        
        let session = serde_json::from_str(&json)
            .map_err(|e| format!("Failed to parse session: {}", e))?;
        
        Ok(Some(session))
    }
}