use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        task: String, // Adds task to json file
    }, 
    Remove {
        task: String, // Remove task from json file
    },
    Done {
        task: String, // Marks tasks as done
    },
    List, // Lists all tasks
}

impl Cli {
    pub fn run(self) {
        match self.command {
            Commands::Add { task } => crate::utils::add_task(&task),
            Commands::Remove { task } => crate::utils::remove_task(&task),
            Commands::List => crate::utils::list_tasks(),
            Commands::Done { task } => crate::utils::mark_as_complete(&task)
        }
    }
}