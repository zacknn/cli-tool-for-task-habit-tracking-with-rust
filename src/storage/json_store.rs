use crate::domain::models::{Habit, PomodoroSession, Task};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::fs;
use std::path::Path;

#[derive(Clone)]
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

    fn save<T: Serialize + ?Sized>(&self, filename: &str, data: &T) -> Result<(), String> {
        let path = format!("{}/{}", self.data_dir, filename);
        let json = serde_json::to_string_pretty(data)
            .map_err(|e| format!("Failed to serialize data: {}", e))?;
        fs::write(&path, json).map_err(|e| format!("Failed to write to file: {}", e))
    }

    fn load<T: DeserializeOwned>(&self, filename: &str) -> Result<Option<T>, String> {
        let path = format!("{}/{}", self.data_dir, filename);
        if !Path::new(&path).exists() {
            return Ok(None);
        }
        let json = fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {}", e))?;
        let data =
            serde_json::from_str(&json).map_err(|e| format!("Failed to parse data: {}", e))?;
        Ok(Some(data))
    }

    // Save tasks
    pub fn save_tasks(&self, tasks: &[Task]) -> Result<(), String> {
        self.save("tasks.json", tasks)
    }

    // Load tasks
    pub fn load_tasks(&self) -> Result<Vec<Task>, String> {
        Ok(self.load("tasks.json")?.unwrap_or_else(Vec::new))
    }

    // Save habits
    pub fn save_habits(&self, habits: &[Habit]) -> Result<(), String> {
        self.save("habits.json", habits)
    }

    // Load habits
    pub fn load_habits(&self) -> Result<Vec<Habit>, String> {
        Ok(self.load("habits.json")?.unwrap_or_else(Vec::new))
    }

    // Save current pomodoro session
    pub fn save_current_session(&self, session: &PomodoroSession) -> Result<(), String> {
        self.save("current_session.json", session)
    }

    // Load current pomodoro session
    pub fn load_current_session(&self) -> Result<Option<PomodoroSession>, String> {
        self.load("current_session.json")
    }
}
