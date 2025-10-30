use std::fs;
use std::path::Path;
use std::str::FromStr;

use anyhow::{Context, Result};
use chrono::{DateTime, Local, NaiveDate};
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

const DB_FILE: &str = "todos.json";
const DATE_FORMAT: &str = "%Y-%m-%d";

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: u64,
    title: String,
    completed: bool,
    created_at: DateTime<Local>,
    due_date: Option<NaiveDate>,
}

impl Todo {
    fn new(id: u64, title: String, due_date: Option<NaiveDate>) -> Self {
        Self {
            id,
            title,
            completed: false,
            created_at: Local::now(),
            due_date,
        }
    }
}

#[derive(Clone, ValueEnum, Debug)]
enum DateInput {
    Today,
    Tomorrow,
    #[clap(skip)]
    Date(NaiveDate),
}

impl FromStr for DateInput {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "today" => Ok(DateInput::Today),
            "tomorrow" => Ok(DateInput::Tomorrow),
            date_str => {
                let date = NaiveDate::parse_from_str(date_str, DATE_FORMAT)?;
                Ok(DateInput::Date(date))
            }
        }
    }
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
    /// Add a new todo with a TITLE and optional due date (format: YYYY-MM-DD, 'today', or 'tomorrow')
    Add {
        title: String,
        /// Due date (format: YYYY-MM-DD, 'today', or 'tomorrow')
        #[arg(short, long, value_parser = parse_date_input)]
        due: Option<DateInput>,
    },
    /// List all todos
    List,
    /// Mark a todo as done by ID
    Done { id: u64 },
    /// Remove a todo by ID
    Remove { id: u64 },
    /// Remove all todos (clear the list)
    Clear,
}

fn parse_date_input(s: &str) -> Result<DateInput, String> {
    s.parse::<DateInput>()
        .map_err(|e| format!("Invalid date format. Use YYYY-MM-DD, 'today', or 'tomorrow': {}", e))
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { title, due } => add_todo(title, due)?,
        Commands::List => list_todos()?,
        Commands::Done { id } => mark_done(id)?,
        Commands::Remove { id } => remove_todo(id)?,
        Commands::Clear => clear_todos()?,
    }

    Ok(())
}

/// Represents the old version of Todo before adding timestamps and due dates
#[derive(Debug, Deserialize)]
struct LegacyTodo {
    id: u64,
    title: String,
    completed: bool,
}

/// Load todos from the JSON file. If the file doesn't exist, return an empty Vec.
/// Handles migration from old todo format to new format.
fn load_todos() -> Result<Vec<Todo>> {
    if !Path::new(DB_FILE).exists() {
        return Ok(vec![]);
    }

    let data = fs::read_to_string(DB_FILE).with_context(|| format!("reading {}", DB_FILE))?;
    
    // First try to parse as the new format
    if let Ok(todos) = serde_json::from_str::<Vec<Todo>>(&data) {
        return Ok(todos);
    }
    
    // If that fails, try to parse as the old format and migrate
    match serde_json::from_str::<Vec<LegacyTodo>>(&data) {
        Ok(legacy_todos) => {
            let migrated_todos: Vec<Todo> = legacy_todos.into_iter().map(|t| {
                Todo {
                    id: t.id,
                    title: t.title,
                    completed: t.completed,
                    created_at: Local::now(), // Set to now for existing todos
                    due_date: None,           // No due date for existing todos
                }
            }).collect();
            
            // Save the migrated todos back to disk
            let json = serde_json::to_string_pretty(&migrated_todos)
                .with_context(|| "serializing migrated todos")?;
            fs::write(DB_FILE, json).with_context(|| format!("writing migrated {}", DB_FILE))?;
            
            Ok(migrated_todos)
        },
        Err(e) => Err(anyhow::anyhow!("Failed to parse todos: {}", e)),
    }
}

/// Save todos to the JSON file (pretty printed)
fn save_todos(todos: &Vec<Todo>) -> Result<()> {
    let json = serde_json::to_string_pretty(todos).with_context(|| "serializing todos to JSON")?;
    fs::write(DB_FILE, json).with_context(|| format!("writing {}", DB_FILE))?;
    Ok(())
}

fn add_todo(title: String, due: Option<DateInput>) -> Result<()> {
    let mut todos = load_todos()?;

    // simple id generation: max id + 1, or 1 if empty
    let new_id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    
    // Convert DateInput to NaiveDate if needed
    let due_date = due.and_then(|d| match d {
        DateInput::Today => Some(chrono::Local::now().date_naive()),
        DateInput::Tomorrow => Some(chrono::Local::now().date_naive() + chrono::Duration::days(1)),
        DateInput::Date(date) => Some(date),
    });
    
    let todo = Todo::new(new_id, title, due_date);
    
    todos.push(todo);
    save_todos(&todos)?;
    
    match due_date {
        Some(date) => println!("Added todo with ID {} (due: {})", new_id, date.format(DATE_FORMAT)),
        None => println!("Added todo with ID {}", new_id),
    }
    
    Ok(())
}

fn list_todos() -> Result<()> {
    let todos = load_todos()?;
    let now = chrono::Local::now();

    if todos.is_empty() {
        println!("No todos yet — add one with `todo-cli add \"Buy milk\"`.");
        return Ok(());
    }

    println!("{:<5} {:<7} {:<30} {:<15} {:<12}", "ID", "Status", "Title", "Created", "Due");
    println!("{:-<5} {:-<7} {:-<30} {:-<15} {:-<12}", "", "", "", "", "");
    
    for t in todos {
        let status = if t.completed { "[✔]" } else { "[ ]" };
        let created_ago = now.signed_duration_since(t.created_at);
        let created_str = if created_ago.num_days() > 0 {
            format!("{}d ago", created_ago.num_days())
        } else if created_ago.num_hours() > 0 {
            format!("{}h ago", created_ago.num_hours())
        } else {
            format!("{}m ago", created_ago.num_minutes().max(1))
        };
        
        let due_str = match t.due_date {
            Some(date) => {
                let _ = date.and_hms_opt(23, 59, 59).unwrap();
                let days_until = (date - now.date_naive()).num_days();
                
                if t.completed {
                    "✓".to_string()
                } else if days_until < 0 {
                    format!("{}d overdue", -days_until)
                } else if days_until == 0 {
                    "Today!".to_string()
                } else if days_until == 1 {
                    "Tomorrow".to_string()
                } else if days_until <= 7 {
                    format!("in {}d", days_until)
                } else {
                    date.format("%b %d").to_string()
                }
            }
            None => "-".to_string(),
        };
        
        println!(
            "{:<5} {:<7} {:<30.27} {:<15} {:<12}",
            t.id, status, t.title, created_str, due_str
        );
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
