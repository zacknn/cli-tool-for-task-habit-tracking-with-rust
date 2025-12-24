use crate::todo::model::Todo;

impl Todo {
    pub fn mark_done(&mut self) {
        self.completed_today = true;
    }

    pub fn print (&self) {
        println!("Todo: {:?}, Completed Today: {:?}", self.title, self.completed_today);
    }
}