mod task;
mod cli;
mod manager;

use std::fs;
use std::path::Path;
use clap::Parser;
use cli::{Command, Cli};
use manager::TodoManager;
use task::Task;

fn main() {
    let path = "data.json";

    if !Path::new(path).exists() {
        fs::write(path, "{}").expect("Failed to create data file!");
    }
    
    let mut todo_manager = TodoManager::new();
    todo_manager.init(path);
    
    let cli = Cli::parse();

    match cli.command {
        Command::Add { name } => {
            todo_manager.add_task(Task::new(&name));
            todo_manager.save();
        }
        Command::List => {
            todo_manager.show_tasks();
        }
        Command::Done { id } => {
            todo_manager.done(id);
            todo_manager.save();
        }
        Command::Delete { id } => {
            todo_manager.delete(id);
            todo_manager.save();
        }
    }
}

