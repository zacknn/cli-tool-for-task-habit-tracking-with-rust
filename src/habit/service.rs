use crate::habit::model::Habit;

impl Habit {
    pub fn mark_done(&mut self) {
        self.streak += 1;
        self.completed_today = true;
    }

    pub fn print(&self) {
        println!(
            "Habit: {:?}, Streak: {:?}, Completed Today: {:?}",
            self.name, self.streak, self.completed_today
        );
    }
}