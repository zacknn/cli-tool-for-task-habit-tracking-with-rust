use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "rtask")]
#[command(about = "CLI for managing Tasks, Habits, and Pomodoro sessions", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: EntityCommands,
}

#[derive(Subcommand, Debug)]
pub enum EntityCommands {
    /// Manage Tasks
    Task(TaskCommand),
    /// Manage Habits
    Habit(HabitCommand),
    /// Manage Pomodoro
    Pomodoro(PomodoroCommand),
}

#[derive(Args, Debug)]
pub struct TaskCommand {
    #[command(subcommand)]
    pub action: CrudActions,
}

#[derive(Args, Debug)]
pub struct HabitCommand {
    #[command(subcommand)]
    pub action: CrudActions,
}

#[derive(Subcommand, Debug)]
pub enum CrudActions {
    /// Create a new item
    Enter(EnterArgs),
    /// Delete an item by ID
    Delete(DeleteArgs),
    /// Read/List items
    Read,
    /// Update an item
    Update(UpdateArgs),
}

#[derive(Args, Debug)]
pub struct EnterArgs {
    /// Title or Name
    #[arg(short, long)]
    pub title: Option<String>,

    /// Description
    #[arg(short, long)]
    pub description: Option<String>,

    /// Frequency (for Habits only)
    #[arg(short, long)]
    pub frequency: Option<String>,
}

#[derive(Args, Debug)]
pub struct DeleteArgs {
    /// ID of the item to delete
    #[arg(short, long)]
    pub id: u32,
}

#[derive(Args, Debug)]
pub struct UpdateArgs {
    /// ID of the item to update
    #[arg(short, long)]
    pub id: u32,

    /// New Title or Name
    #[arg(short, long)]
    pub title: Option<String>,

    /// New Description
    #[arg(short, long)]
    pub description: Option<String>,

    /// Mark as completed (for Tasks)
    #[arg(short, long)]
    pub completed: bool,

    /// Increment streak (for Habits)
    #[arg(short, long)]
    pub increment: bool,
}

#[derive(Args, Debug)]
pub struct PomodoroCommand {
    #[command(subcommand)]
    pub action: PomodoroActions,
}

#[derive(Subcommand, Debug)]
pub enum PomodoroActions {
    /// Start a new pomodoro session
    Enter(StartPomodoroArgs),
    /// Stop current session
    Delete,
    /// Check status
    Read,
    // Update not strictly needed but could be pause/resume? keeping simple as per req
}

#[derive(Args, Debug)]
pub struct StartPomodoroArgs {
    /// Work duration in minutes
    #[arg(short, long, default_value = "25")]
    pub work: u32,

    /// Short break duration
    #[arg(short = 'b', long, default_value = "5")]
    pub short_break: u32,

    /// Long break duration
    #[arg(short = 'B', long, default_value = "15")]
    pub long_break: u32,

    /// Total sessions
    #[arg(short = 's', long, default_value = "6")]
    pub sessions: u32,

    /// Sessions before long break
    #[arg(short = 'l', long, default_value = "4")]
    pub long_break_interval: u32,

    /// Associate with Task ID
    #[arg(short = 't', long)]
    pub task_id: Option<u32>,
}
