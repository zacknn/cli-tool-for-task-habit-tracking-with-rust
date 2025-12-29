mod cli;
mod domain;
mod storage;
mod app;

use clap::Parser;
use cli::args::Cli;
use domain::models::{Task, PomodoroSession};
use storage::json_store::JsonStorage;
use domain::services::pomodoro_service::PomodoroService;
use std::io::{self, Write};

fn main() {
    let cli = Cli::parse();
    
    // Initialize storage
    let storage = JsonStorage::new("./data");
    let pomodoro_service = PomodoroService::new(storage);
    
    match cli.command {
        cli::args::Commands::Start(args) => {
            start_pomodoro(args, &pomodoro_service);
        }
        cli::args::Commands::Tasks => {
            list_tasks(&pomodoro_service);
        }
        cli::args::Commands::Status => {
            show_status(&pomodoro_service);
        }
        cli::args::Commands::Stop => {
            stop_session(&pomodoro_service);
        }
    }
}

fn start_pomodoro(args: cli::args::StartArgs, service: &PomodoroService) {
    println!("Starting Pomodoro Session...");
    println!("Work: {} min, Short Break: {} min, Long Break: {} min", 
             args.work, args.short_break, args.long_break);
    println!("Total Sessions: {}, Long Break after: {}", 
             args.sessions, args.sessions_before_long);
    
    // Ask for task (optional)
    let task = if !args.no_task {
        ask_for_task()
    } else {
        None
    };
    
    // Start session
    match service.start_session(
        args.work,
        args.short_break,
        args.long_break,
        args.sessions,
        args.sessions_before_long,
        task,
    ) {
        Ok(session) => {
            println!(" Session started!");
            println!("Session {} of {}", session.current_session, session.total_sessions);
            if let Some(task_id) = session.task_id {
                println!("Associated with Task ID: {}", task_id);
            }
            
            // Start the timer loop
            run_pomodoro_timer(service, session);
        }
        Err(e) => {
            eprintln!(" Failed to start session: {}", e);
        }
    }
}

fn ask_for_task() -> Option<Task> {
    println!("\nWould you like to add a task for this pomodoro? (y/n)");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    if input.trim().to_lowercase() != "y" {
        return None;
    }
    
    println!("Enter task title:");
    io::stdout().flush().unwrap();
    
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();
    let title = title.trim().to_string();
    
    if title.is_empty() {
        println!("Task title cannot be empty. Skipping task creation.");
        return None;
    }
    
    println!("Enter task description (optional):");
    io::stdout().flush().unwrap();
    
    let mut description = String::new();
    io::stdin().read_line(&mut description).unwrap();
    let description = if description.trim().is_empty() {
        None
    } else {
        Some(description.trim().to_string())
    };
    
    // Simple ID generation
    let id = chrono::Utc::now().timestamp() as u32 % 10000;
    
    Some(Task {
        id,
        title,
        description,
        created_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    })
}

fn run_pomodoro_timer(service: &PomodoroService, session: PomodoroSession) {
    println!("\nPomodoro Timer Started!");
    println!("Press Enter to complete current session, or Ctrl+C to stop.");
    
    // Simple timer logic 
    // In a real app, you'd use async timers and proper notifications
    
    for session_num in session.current_session..=session.total_sessions {
        println!("\n Session {}/{} - WORK TIME!", session_num, session.total_sessions);
        println!("Focus for {} minutes...", session.work_duration);
        
        // Wait for user to press Enter (simulating timer)
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        // Complete work session
        match service.complete_work_session() {
            Ok(updated_session) => {
                println!(" Work session {} completed!", session_num);
                
                // Check if we need a break
                if session_num < session.total_sessions {
                    let break_type = if session_num % session.sessions_before_long_break == 0 {
                        "LONG"
                    } else {
                        "SHORT"
                    };
                    
                    let break_duration = if session_num % session.sessions_before_long_break == 0 {
                        session.long_break
                    } else {
                        session.short_break
                    };
                    
                    println!("\n☕ {} BREAK - {} minutes", break_type, break_duration);
                    println!("Press Enter when break is over...");
                    
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    
                    // Complete break
                    match service.complete_break() {
                        Ok(_) => println!("Break completed!"),
                        Err(e) => eprintln!("❌ Error: {}", e),
                    }
                } else {
                    println!("\n Congratulations! All sessions completed!");
                    let _ = service.stop_session();
                    break;
                }
            }
            Err(e) => {
                eprintln!(" Error: {}", e);
                break;
            }
        }
    }
}

fn list_tasks(service: &PomodoroService) {
    match service.list_tasks() {
        Ok(tasks) => {
            if tasks.is_empty() {
                println!("No tasks found.");
            } else {
                println!(" Your Tasks:");
                for task in tasks {
                    println!("  ID: {}", task.id);
                    println!("  Title: {}", task.title);
                    if let Some(desc) = task.description {
                        println!("  Description: {}", desc);
                    }
                    println!("  Created: {}", task.created_at);
                    println!("  ---");
                }
            }
        }
        Err(e) => eprintln!(" Error loading tasks: {}", e),
    }
}

fn show_status(service: &PomodoroService) {
    match service.get_status() {
        Ok(Some(session)) => {
            println!(" Current Pomodoro Status:");
            println!("  Session: {}/{}", session.current_session, session.total_sessions);
            println!("  Currently: {}", if session.is_break { "On Break" } else { "Working" });
            println!("  Work Duration: {} min", session.work_duration);
            println!("  Completed: {} sessions", session.completed_sessions.len());
            
            if let Some(task_id) = session.task_id {
                println!("  Associated Task ID: {}", task_id);
            }
        }
        Ok(None) => println!("No active pomodoro session."),
        Err(e) => eprintln!(" Error: {}", e),
    }
}

fn stop_session(service: &PomodoroService) {
    match service.stop_session() {
        Ok(_) => println!(" Session stopped."),
        Err(e) => eprintln!(" Error: {}", e),
    }
}