use clap::{Parser, Subcommand, ValueEnum};
use crate::models::{DateInput, Priority};
use anyhow::Result;
use chrono::NaiveDate;

/// Command line interface for the todo application
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands
#[derive(Subcommand)]
pub enum Commands {
    /// Add a new todo
    Add {
        /// The title of the todo
        title: String,
        
        /// Due date (today, tomorrow, or YYYY-MM-DD)
        #[arg(short, long, value_parser = parse_date_input)]
        due: Option<DateInput>,
        
        /// Priority level
        #[arg(short, long, value_enum, default_value_t = Priority::Normal)]
        priority: Priority,
    },
    
    /// List todos
    List {
        /// Sort order
        #[arg(short, long, value_enum, default_value_t = SortOrder::Smart)]
        sort: SortOrder,
        
        /// Show only active (incomplete) todos
        #[arg(short, long)]
        active: bool,
        
        /// Filter by minimum priority
        #[arg(short, long, value_enum)]
        priority: Option<Priority>,
    },
    
    /// Mark a todo as done
    Done {
        /// ID of the todo to mark as done
        id: u64,
    },
    
    /// Remove a todo
    Remove {
        /// ID of the todo to remove
        id: u64,
    },
    
    /// Clear all todos
    Clear,
    
    /// Set priority of a todo
    Priority {
        /// ID of the todo
        id: u64,
        
        /// New priority level
        priority: Priority,
    },
}

/// Available sort orders for listing todos
#[derive(ValueEnum, Clone, Debug)]
pub enum SortOrder {
    /// Smart sorting (incomplete first, then by priority, due date, and creation time)
    Smart,
    
    /// Sort by due date (earliest first)
    Due,
    
    /// Sort by priority (highest first)
    Priority,
    
    /// Sort by creation time (oldest first)
    Created,
}

/// Parse a date string into a DateInput enum
pub fn parse_date_input(s: &str) -> Result<DateInput, String> {
    match s.to_lowercase().as_str() {
        "today" => Ok(DateInput::Today),
        "tomorrow" => Ok(DateInput::Tomorrow),
        _ => {
            NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .map(DateInput::Date)
                .map_err(|_| format!("Invalid date format. Use 'today', 'tomorrow', or 'YYYY-MM-DD'"))
        }
    }
}

/// Parse command line arguments
pub fn parse() -> Cli {
    Cli::parse()
}
