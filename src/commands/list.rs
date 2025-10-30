use crate::{
    models::Priority,
    storage::load_todos,
    display::display_todos,
};
use super::CommandResult;

/// Lists todos with optional filtering and sorting
pub fn list_todos(sort_order: crate::cli::SortOrder, active_only: bool, min_priority: Option<Priority>) -> CommandResult {
    let mut todos = load_todos()?;
    
    // Apply filters
    if active_only {
        todos.retain(|t| !t.completed);
    }
    
    if let Some(min_prio) = min_priority {
        todos.retain(|t| t.priority >= min_prio);
    }
    
    // Apply sorting
    match sort_order {
        crate::cli::SortOrder::Smart => {
            // Already implemented via the Ord trait
            todos.sort();
        }
        crate::cli::SortOrder::Due => {
            todos.sort_by(|a, b| {
                match (a.due_date, b.due_date) {
                    (Some(a_date), Some(b_date)) => a_date.cmp(&b_date),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => a.cmp(b),
                }
            });
        }
        crate::cli::SortOrder::Priority => {
            todos.sort_by(|a, b| {
                b.priority.cmp(&a.priority)
                    .then_with(|| a.cmp(b))
            });
        }
        crate::cli::SortOrder::Created => {
            todos.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        }
    }
    
    // Display the todos
    display_todos(&todos);
    
    Ok(())
}
