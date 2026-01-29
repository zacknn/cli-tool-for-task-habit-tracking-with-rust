use crate::domain::models::Task;
use crate::storage::json_store::JsonStorage;
use chrono::Local;

pub struct TaskService {
    storage: JsonStorage,
}

impl TaskService {
    pub fn new(storage: JsonStorage) -> Self {
        Self { storage }
    }

    pub fn create_task(&self, title: String, description: Option<String>) -> Result<Task, String> {
        let mut tasks = self.storage.load_tasks()?;

        let id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;

        let new_task = Task {
            id,
            title,
            description,
            created_at: Local::now(),
            completed: false,
            completed_at: None,
        };

        tasks.push(new_task.clone());
        self.storage.save_tasks(&tasks)?;

        Ok(new_task)
    }

    pub fn list_tasks(&self) -> Result<Vec<Task>, String> {
        self.storage.load_tasks()
    }

    pub fn delete_task(&self, id: u32) -> Result<(), String> {
        let mut tasks = self.storage.load_tasks()?;
        if let Some(pos) = tasks.iter().position(|t| t.id == id) {
            tasks.remove(pos);
            self.storage.save_tasks(&tasks)?;
            Ok(())
        } else {
            Err(format!("Task with ID {} not found", id))
        }
    }

    pub fn update_task(
        &self,
        id: u32,
        title: Option<String>,
        description: Option<String>,
        completed: Option<bool>,
    ) -> Result<Task, String> {
        let mut tasks = self.storage.load_tasks()?;

        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            if let Some(t) = title {
                task.title = t;
            }
            if let Some(d) = description {
                task.description = Some(d);
            }
            if let Some(c) = completed {
                task.completed = c;
                if c {
                    task.completed_at = Some(Local::now());
                } else {
                    task.completed_at = None;
                }
            }
            let __updated_task = task.clone();
            // multiple immutable borrows due to save_tasks needing &tasks later?
            // no, we mutate `tasks`, then save it.
            // slice needs to be taken from `tasks`.
        } else {
            return Err(format!("Task with ID {} not found", id));
        }

        // Must re-find or scope the borrow if we want to return the task and save
        // Actually, we can clone the found task inside the match, but we modified it in place.
        // Let's just save.
        self.storage.save_tasks(&tasks)?;

        // Retrieve again to return fresh? Or just find again?
        // Since we modified in place, we can find it again.
        let updated_task = tasks
            .iter()
            .find(|t| t.id == id)
            .cloned()
            .ok_or("Task lost")?;
        Ok(updated_task)
    }
}
