use crate::models::*;
use chrono::{DateTime, Local, NaiveDate};
use colored::*;

const DATE_FORMAT: &str = "%b %-d";

/// Formats a datetime as a relative time string (e.g., "2h ago")
pub fn format_relative_time(dt: &DateTime<Local>) -> String {
    let now = Local::now();
    let duration = now.signed_duration_since(*dt);
    
    if duration.num_days() > 30 {
        dt.format("%b %-d, %Y").to_string()
    } else if duration.num_days() > 0 {
        format!("{}d ago", duration.num_days())
    } else if duration.num_hours() > 0 {
        format!("{}h ago", duration.num_hours())
    } else if duration.num_minutes() > 0 {
        format!("{}m ago", duration.num_minutes())
    } else {
        "just now".to_string()
    }
}

/// Formats a due date with relative indicators (e.g., "Tomorrow", "3d overdue")
pub fn format_due_date(due_date: Option<NaiveDate>) -> String {
    match due_date {
        Some(date) => {
            let today = Local::now().date_naive();
            let days_until = (date - today).num_days();
            
            match days_until {
                0 => "Today".to_string(),
                1 => "Tomorrow".to_string(),
                2..=6 => format!("in {}d", days_until),
                _ if days_until > 6 => date.format(DATE_FORMAT).to_string(),
                _ => format!("{}d overdue", -days_until).red().to_string(),
            }
        }
        None => "-".to_string(),
    }
}

/// Formats a todo's status (completed or not)
pub fn format_status(completed: bool) -> String {
    if completed {
        "[✔]".green().to_string()
    } else {
        "[ ]".to_string()
    }
}

/// Formats a priority with color coding
pub fn format_priority(priority: Priority) -> String {
    match priority {
        Priority::Low => priority.to_string().dimmed().to_string(),
        Priority::Normal => priority.to_string(),
        Priority::High => priority.to_string().yellow().to_string(),
        Priority::Urgent => priority.to_string().red().bold().to_string(),
    }
}

/// Helper trait for displaying todos in different formats
pub trait TodoDisplay {
    fn display(&self) -> String;
}

impl TodoDisplay for Todo {
    fn display(&self) -> String {
        let status = format_status(self.completed);
        let priority = format_priority(self.priority);
        let created = format_relative_time(&self.created_at);
        let due = format_due_date(self.due_date);
        
        format!(
            "{:<5} {:<7} {:<8} {:<30} {:<14} {}",
            self.id, status, priority, self.title, created, due
        )
    }
}

/// Displays a list of todos with a header
pub fn display_todos(todos: &[Todo]) {
    if todos.is_empty() {
        println!("No todos found.");
        return;
    }
    
    println!(
        "{:<5} {:<7} {:<8} {:<30} {:<14} {}",
        "ID", "Status", "Priority", "Title", "Created", "Due"
    );
    println!("{}", "-".repeat(80));
    
    for todo in todos {
        println!("{}", todo.display());
    }
}
