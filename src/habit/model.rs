#[derive(Debug)]
pub enum Frequency {
    Daily,
    Weekly,
}
#[derive(Debug)]
pub struct Habit {
    pub name: String,
    pub streak: u32,
    pub completed_today: bool,
    pub frequency: Frequency,
}