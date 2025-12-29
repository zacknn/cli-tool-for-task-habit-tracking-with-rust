use clap::{Parser, Subcommand, Args};


#[derive(Parser, Debug)]
#[command(name = "pomodoro")]
#[command(about = "A simple pomodoro timer", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand , Debug)]
pub enum Commands {
    // start a new pomodoro
    Start(StartArgs),
    // stop current pomodoro
    Stop,
    // status of current pomodoro
    Status,
    // see all tasks
    Tasks,
}

#[derive(Args , Debug)]
pub struct StartArgs {
    /// Work duration in minutes (default: 25)
    #[arg(short, long, default_value = "25")]
    pub work: u32,
    
    /// Short break duration in minutes (default: 5)
    #[arg(short = 'b', long, default_value = "5")]
    pub short_break: u32,
    
    /// Long break duration in minutes (default: 15)
    #[arg(short = 'B', long, default_value = "15")]
    pub long_break: u32,
    
    /// Total sessions to complete (default: 6)
    #[arg(short, long, default_value = "6")]
    pub sessions: u32,
    
    /// Sessions before long break (default: 4)
    #[arg(short = 'l', long, default_value = "4")]
    pub sessions_before_long: u32,
    
    /// Skip task creation
    #[arg(short, long)]
    pub no_task: bool,
}