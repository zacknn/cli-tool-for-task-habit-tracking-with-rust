use crate::domain::models::Habit;
use crate::storage::json_store::JsonStorage;
use chrono::Local;

pub struct HabitService {
    storage: JsonStorage,
}

impl HabitService {
    pub fn new(storage: JsonStorage) -> Self {
        Self { storage }
    }

    pub fn create_habit(
        &self,
        name: String,
        description: Option<String>,
        frequency: String,
    ) -> Result<Habit, String> {
        let mut habits = self.storage.load_habits()?;

        let id = habits.iter().map(|h| h.id).max().unwrap_or(0) + 1;

        let new_habit = Habit {
            id,
            name,
            description,
            frequency,
            streak: 0,
            created_at: Local::now(),
            last_completed_at: None,
        };

        habits.push(new_habit.clone());
        self.storage.save_habits(&habits)?;

        Ok(new_habit)
    }

    pub fn list_habits(&self) -> Result<Vec<Habit>, String> {
        self.storage.load_habits()
    }

    pub fn delete_habit(&self, id: u32) -> Result<(), String> {
        let mut habits = self.storage.load_habits()?;
        if let Some(pos) = habits.iter().position(|h| h.id == id) {
            habits.remove(pos);
            self.storage.save_habits(&habits)?;
            Ok(())
        } else {
            Err(format!("Habit with ID {} not found", id))
        }
    }

    pub fn update_habit(
        &self,
        id: u32,
        name: Option<String>,
        description: Option<String>,
        frequency: Option<String>,
        increment_streak: bool,
    ) -> Result<Habit, String> {
        let mut habits = self.storage.load_habits()?;

        if let Some(habit) = habits.iter_mut().find(|h| h.id == id) {
            if let Some(n) = name {
                habit.name = n;
            }
            if let Some(d) = description {
                habit.description = Some(d);
            }
            if let Some(f) = frequency {
                habit.frequency = f;
            }
            if increment_streak {
                habit.streak += 1;
                habit.last_completed_at = Some(Local::now());
            }
        } else {
            return Err(format!("Habit with ID {} not found", id));
        }

        self.storage.save_habits(&habits)?;

        let updated_habit = habits
            .iter()
            .find(|h| h.id == id)
            .cloned()
            .ok_or("Habit lost")?;
        Ok(updated_habit)
    }
}
