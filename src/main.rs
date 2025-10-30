use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

const DB_FILE: &str = "todos.json";

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: u64,
    title: String,
    completed: bool,
}

#[derive(Parser)]
#[command(name = "todo-cli")]
#[command(about = "A tiny CLI todo app in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new todo with a TITLE
    Add { title: String },
    /// List all todos
    List,
    /// Mark a todo as done by ID
    Done { id: u64 },
    /// Remove a todo by ID
    Remove { id: u64 },
    /// Remove all todos (clear the list)
    Clear,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { title } => add_todo(title)?,
        Commands::List => list_todos()?,
        Commands::Done { id } => mark_done(id)?,
        Commands::Remove { id } => remove_todo(id)?,
        Commands::Clear => clear_todos()?,
    }

    Ok(())
}

/// Load todos from the JSON file. If the file doesn't exist, return an empty Vec.
fn load_todos() -> Result<Vec<Todo>> {
    if !Path::new(DB_FILE).exists() {
        return Ok(vec![]);
    }

    let data = fs::read_to_string(DB_FILE).with_context(|| format!("reading {}", DB_FILE))?;
    let todos: Vec<Todo> = serde_json::from_str(&data).with_context(|| "parsing JSON")?;
    Ok(todos)
}

/// Save todos to the JSON file (pretty printed)
fn save_todos(todos: &Vec<Todo>) -> Result<()> {
    let json = serde_json::to_string_pretty(todos).with_context(|| "serializing todos to JSON")?;
    fs::write(DB_FILE, json).with_context(|| format!("writing {}", DB_FILE))?;
    Ok(())
}

fn add_todo(title: String) -> Result<()> {
    let mut todos = load_todos()?;

    // simple id generation: max id + 1, or 1 if empty
    let new_id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let todo = Todo {
        id: new_id,
        title,
        completed: false,
    };

    todos.push(todo);
    save_todos(&todos)?;
    println!("Added todo with ID {}", new_id);
    Ok(())
}

fn list_todos() -> Result<()> {
    let todos = load_todos()?;

    if todos.is_empty() {
        println!("No todos yet — add one with `todo-cli add \"Buy milk\"`.");
        return Ok(());
    }

    println!("ID Status Title");
    println!("-- ------ -----");
    for t in todos {
        let status = if t.completed { "✔" } else { " " };
        println!("{:<3} [{:^1}] {}", t.id, status, t.title);
    }

    Ok(())
}

fn mark_done(id: u64) -> Result<()> {
    let mut todos = load_todos()?;
    let mut found = false;
    for t in &mut todos {
        if t.id == id {
            t.completed = true;
            found = true;
            break;
        }
    }

    if !found {
        println!("Todo with ID {} not found.", id);
        return Ok(());
    }

    save_todos(&todos)?;
    println!("Marked {} as done.", id);
    Ok(())
}

fn remove_todo(id: u64) -> Result<()> {
    let mut todos = load_todos()?;
    let original_len = todos.len();
    todos.retain(|t| t.id != id);

    if todos.len() == original_len {
        println!("Todo with ID {} not found.", id);
        return Ok(());
    }

    save_todos(&todos)?;
    println!("Removed todo {}.", id);
    Ok(())
}

fn clear_todos() -> Result<()> {
    // Overwrite with an empty list
    save_todos(&vec![])?;
    println!("All todos removed.");
    Ok(())
}
