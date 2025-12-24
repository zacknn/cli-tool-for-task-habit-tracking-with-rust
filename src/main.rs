mod habit;
use habit::model::{Frequency, Habit};

fn main() {
    let mut habits = Vec::new();

    habits.push(Habit {
        name: String::from("Code 1 hour"),
        streak: 5,
        completed_today: false,
        frequency: Frequency::Daily,
    });

    habits.push(Habit {
        name: String::from("Exercise"),
        streak: 10,
        completed_today: false,
        frequency: Frequency::Weekly,
    });

    habits.get_mut(0).unwrap().mark_done();

    for habit in &habits {
        habit.print();
    }
}
