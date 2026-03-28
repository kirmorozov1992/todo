use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo", about = "A simple todo CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Add new task
    Add {
        name: String,
    },
    /// Get task list
    List,
    /// Change task status to Done by id
    Done {
        id: u32
    },
    /// Delete task by id
    Delete {
        id: u32
    },
}

