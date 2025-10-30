use std::fs;
use std::path::Path;
use std::str::FromStr;

use anyhow::{Context, Result};
use chrono::{DateTime, Local, NaiveDate};
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

const DB_FILE: &str = "todos.json";
const DATE_FORMAT: &str = "%Y-%m-%d";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
enum Priority {
    /// Low priority task
    Low,
    /// Normal priority task (default)
    Normal,
    /// High priority task
    High,
    /// Urgent priority task
    Urgent,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Normal
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "Low"),
            Priority::Normal => write!(f, "Normal"),
            Priority::High => write!(f, "High"),
            Priority::Urgent => write!(f, "URGENT"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: u64,
    title: String,
    completed: bool,
    created_at: DateTime<Local>,
    due_date: Option<NaiveDate>,
    #[serde(default)]
    priority: Priority,
}

impl Todo {
    fn priority_value(&self) -> u8 {
        match self.priority {
            Priority::Low => 1,
            Priority::Normal => 2,
            Priority::High => 3,
            Priority::Urgent => 4,
        }
    }
}

impl PartialEq for Todo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Todo {}

impl PartialOrd for Todo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Todo {
    fn cmp(&self, other: &Self) -> Ordering {
        // First by completion status (incomplete first)
        self.completed
            .cmp(&other.completed)
            .reverse()
            // Then by priority (highest first)
            .then_with(|| self.priority_value().cmp(&other.priority_value()).reverse())
            // Then by due date (earlier first, None last)
            .then_with(|| {
                match (self.due_date, other.due_date) {
                    (Some(a), Some(b)) => a.cmp(&b),
                    (Some(_), None) => Ordering::Less,
                    (None, Some(_)) => Ordering::Greater,
                    (None, None) => Ordering::Equal,
                }
            })
            // Finally by creation date (oldest first)
            .then_with(|| self.created_at.cmp(&other.created_at))
    }
}

impl Todo {
    fn new(id: u64, title: String, due_date: Option<NaiveDate>, priority: Priority) -> Self {
        Self {
            id,
            title,
            completed: false,
            created_at: Local::now(),
            due_date,
            priority,
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
    /// Add a new todo
    Add {
        /// The task description
        title: String,
        
        /// Due date (format: YYYY-MM-DD, 'today', or 'tomorrow')
        #[arg(short, long, value_parser = parse_date_input)]
        due: Option<DateInput>,
        
        /// Priority level [default: normal]
        #[arg(short, long, value_enum, default_value_t = Priority::Normal)]
        priority: Priority,
    },
    
    /// List all todos
    List {
        /// Sort order [default: smart]
        #[arg(short, long, value_enum, default_value_t = SortOrder::Smart)]
        sort: SortOrder,
        
        /// Show only uncompleted tasks
        #[arg(short, long)]
        active: bool,
        
        /// Filter by priority or higher
        #[arg(short = 'P', long, value_enum)]
        priority: Option<Priority>,
    },
    
    /// Mark a todo as done by ID
    Done {
        id: u64,
    },
    
    /// Remove a todo by ID
    Remove {
        id: u64,
    },
    
    /// Remove all todos (clear the list)
    Clear,
    
    /// Set priority of a todo
    Priority {
        /// Todo ID
        id: u64,
        
        /// New priority level
        #[arg(value_enum)]
        priority: Priority,
    },
}

fn parse_date_input(s: &str) -> Result<DateInput, String> {
    s.parse::<DateInput>()
        .map_err(|e| format!("Invalid date format. Use YYYY-MM-DD, 'today', or 'tomorrow': {}", e))
}

/// Sort order for listing todos
#[derive(ValueEnum, Clone, Debug)]
enum SortOrder {
    /// Smart sorting: incomplete first, then by priority, due date, and creation time
    Smart,
    
    /// Sort by due date (earliest first)
    Due,
    
    /// Sort by priority (highest first)
    Priority,
    
    /// Sort by creation time (oldest first)
    Created,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { title, due, priority } => add_todo(title, due, priority)?,
        Commands::List { sort, active, priority } => list_todos(sort, active, priority)?,
        Commands::Done { id } => mark_done(id)?,
        Commands::Remove { id } => remove_todo(id)?,
        Commands::Clear => clear_todos()?,
        Commands::Priority { id, priority } => set_priority(id, priority)?,
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
                    priority: Priority::Normal, // Default priority for existing todos
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

fn add_todo(title: String, due: Option<DateInput>, priority: Priority) -> Result<()> {
    let mut todos = load_todos()?;

    // simple id generation: max id + 1, or 1 if empty
    let new_id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    
    // Convert DateInput to NaiveDate if needed
    let due_date = due.and_then(|d| match d {
        DateInput::Today => Some(chrono::Local::now().date_naive()),
        DateInput::Tomorrow => Some(chrono::Local::now().date_naive() + chrono::Duration::days(1)),
        DateInput::Date(date) => Some(date),
    });
    
    // Create the todo with all fields
    let todo = Todo::new(new_id, title, due_date, priority);
    
    // Clone values needed for the success message before moving todo
    let title_clone = todo.title.clone();
    let priority_clone = todo.priority;
    
    todos.push(todo);
    save_todos(&todos)?;
    
    match due_date {
        Some(date) => println!("Added todo #{} '{}' (Priority: {}, Due: {})", 
            new_id, title_clone, priority_clone, date.format(DATE_FORMAT)),
        None => println!("Added todo #{} '{}' (Priority: {})", 
            new_id, title_clone, priority_clone),
    }
    
    Ok(())
}

fn sort_todos(mut todos: Vec<Todo>, sort_order: &SortOrder) -> Vec<Todo> {
    match sort_order {
        SortOrder::Smart => {
            // Already implemented via the Ord trait
            todos.sort();
        }
        SortOrder::Due => {
            todos.sort_by(|a, b| {
                match (a.due_date, b.due_date) {
                    (Some(a_date), Some(b_date)) => a_date.cmp(&b_date),
                    (Some(_), None) => Ordering::Less,
                    (None, Some(_)) => Ordering::Greater,
                    (None, None) => a.created_at.cmp(&b.created_at),
                }
            });
        }
        SortOrder::Priority => {
            todos.sort_by(|a, b| {
                b.priority_value()
                    .cmp(&a.priority_value())
                    .then_with(|| a.created_at.cmp(&b.created_at))
            });
        }
        SortOrder::Created => {
            todos.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        }
    }
    todos
}

fn list_todos(sort_order: SortOrder, active_only: bool, min_priority: Option<Priority>) -> Result<()> {
    let mut todos = load_todos()?;
    let now = chrono::Local::now();

    // Apply filters
    if active_only {
        todos.retain(|t| !t.completed);
    }
    
    if let Some(min_pri) = min_priority {
        let min_value = match min_pri {
            Priority::Low => 1,
            Priority::Normal => 2,
            Priority::High => 3,
            Priority::Urgent => 4,
        };
        todos.retain(|t| t.priority_value() >= min_value);
    }

    if todos.is_empty() {
        println!("No todos found matching your criteria.");
        return Ok(());
    }

    // Sort todos according to the specified order
    let todos = sort_todos(todos, &sort_order);

    println!("{:<5} {:<7} {:<8} {:<30} {:<15} {:<12}", 
        "ID", "Status", "Priority", "Title", "Created", "Due");
    println!("{:-<5} {:-<7} {:-<8} {:-<30} {:-<15} {:-<12}", 
        "", "", "", "", "", "");
    
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
        
        // Color the priority based on its level
        let priority_str = match t.priority {
            Priority::Low => format!("{}", t.priority),
            Priority::Normal => format!("{}", t.priority),
            Priority::High => format!("\x1b[33m{}\x1b[0m", t.priority),  // Yellow
            Priority::Urgent => format!("\x1b[31m{}\x1b[0m", t.priority), // Red
        };
        
        let title = if t.completed {
            format!("\x1b[2m{}\x1b[0m", t.title)  // Dim completed tasks
        } else {
            t.title.clone()
        };
        
        println!(
            "{:<5} {:<7} {:<8} {:<30.27} {:<15} {:<12}",
            t.id, status, priority_str, title, created_str, due_str
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

fn set_priority(id: u64, priority: Priority) -> Result<()> {
    let mut todos = load_todos()?;
    let mut found = false;
    
    for todo in &mut todos {
        if todo.id == id {
            let old_priority = std::mem::replace(&mut todo.priority, priority);
            println!("Changed priority of todo #{} from {} to {}", 
                id, old_priority, todo.priority);
            found = true;
            break;
        }
    }
    
    if !found {
        return Err(anyhow::anyhow!("Todo with ID {} not found", id));
    }
    
    save_todos(&todos)?;
    Ok(())
}

fn clear_todos() -> Result<()> {
    // Overwrite with an empty list
    save_todos(&vec![])?;
    println!("All todos removed.");
    Ok(())
}
