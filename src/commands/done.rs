use crate::{
    storage::{load_todos, save_todos},
    display::display_todos,
};
use super::CommandResult;

/// Marks a todo as done by its ID
pub fn mark_done(id: u64) -> CommandResult {
    let mut todos = load_todos()?;
    let mut found = false;
    let mut todo_title = String::new();
    
    // First pass: find and update the todo
    for todo in &mut todos {
        if todo.id == id {
            if todo.completed {
                println!("Todo #{} is already marked as done.", id);
                return Ok(());
            } else {
                todo.completed = true;
                todo_title = todo.title.clone();
                found = true;
                break;
            }
        }
    }
    
    if !found {
        return Err(anyhow::anyhow!("Todo #{} not found", id));
    }
    
    // Save the updated todos
    save_todos(&todos)?;
    println!("Marked todo #{} as done: {}", id, todo_title);
    
    // Show the updated list
    display_todos(&todos);
    
    Ok(())
}
