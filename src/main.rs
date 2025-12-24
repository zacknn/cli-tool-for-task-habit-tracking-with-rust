mod habit;
use habit::model::{Frequency, Habit};
mod todo;
use todo::model::Todo;
fn main() {

    let mut habits = Vec::new();
    let mut todos: Vec<Todo> = Vec::new();


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

    todos.push( Todo { title:String::from("read 10 pages of a book"), completed_today: (false) });
    todos.push( Todo { title:String::from("code a project"), completed_today: (false) });
    todos.push( Todo { title:String::from("animate with python"), completed_today: (false) });

    for todo in &todos {
        todo.print();
    }


}
