//! A command-line todo list application with priorities and due dates.

#![warn(missing_docs)]

use anyhow::Result;
use todo_cli::{
    cli::parse,
    commands::{
        add_todo, clear_todos, list_todos, mark_done, remove_todo, set_priority
    },
};

fn main() -> Result<()> {
    let cli = parse();

    match cli.command {
        todo_cli::cli::Commands::Add { title, due, priority } => {
            add_todo(title, due, priority)
        }
        todo_cli::cli::Commands::List { sort, active, priority } => {
            list_todos(sort, active, priority)
        }
        todo_cli::cli::Commands::Done { id } => {
            mark_done(id)
        }
        todo_cli::cli::Commands::Remove { id } => {
            remove_todo(id)
        }
        todo_cli::cli::Commands::Clear => {
            clear_todos()
        }
        todo_cli::cli::Commands::Priority { id, priority } => {
            set_priority(id, priority)
        }
    }
}
