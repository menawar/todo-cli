use crate::{
    models::{DateInput, Todo},
    storage::{load_todos, save_todos},
    display::display_todos,
};
use anyhow::Result;
use chrono::{Local, Duration};

/// Adds a new todo with the given title, due date, and priority
pub fn add_todo(title: String, due: Option<DateInput>, priority: crate::models::Priority) -> Result<()> {
    let mut todos = load_todos()?;
    
    // Generate a new ID (max ID + 1)
    let new_id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    
    // Convert DateInput to NaiveDate if needed
    let due_date = due.and_then(|d| match d {
        DateInput::Today => Some(Local::now().date_naive()),
        DateInput::Tomorrow => Some(Local::now().date_naive() + Duration::days(1)),
        DateInput::Date(date) => Some(date),
    });
    
    // Create the todo with all fields
    let todo = Todo::new(new_id, title, due_date, priority);
    
    // Clone values needed for the success message before moving todo
    let title_clone = todo.title.clone();
    let priority_clone = todo.priority;
    
    todos.push(todo);
    save_todos(&todos)?;
    
    // Display success message with appropriate formatting
    match due_date {
        Some(date) => println!("Added todo #{} '{}' (Priority: {}, Due: {})", 
            new_id, title_clone, priority_clone, date.format("%Y-%m-%d")),
        None => println!("Added todo #{} '{}' (Priority: {})", 
            new_id, title_clone, priority_clone),
    }
    
    // Show the updated list
    display_todos(&todos);
    
    Ok(())
}
