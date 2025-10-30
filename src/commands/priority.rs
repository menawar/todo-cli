use crate::{
    models::Priority,
    storage::{load_todos, save_todos},
    display::display_todos,
};
use super::CommandResult;

/// Updates the priority of a todo
pub fn set_priority(id: u64, new_priority: Priority) -> CommandResult {
    let mut todos = load_todos()?;
    
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        if todo.priority == new_priority {
            println!("Todo #{} already has priority: {}", id, new_priority);
        } else {
            let old_priority = std::mem::replace(&mut todo.priority, new_priority);
            save_todos(&todos)?;
            println!("Updated priority of todo #{} from {} to {}", 
                     id, old_priority, new_priority);
        }
    } else {
        return Err(anyhow::anyhow!("Todo #{} not found", id));
    }
    
    // Show the updated list
    display_todos(&todos);
    
    Ok(())
}
