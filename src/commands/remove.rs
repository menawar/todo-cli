use crate::{
    storage::{load_todos, save_todos},
    display::display_todos,
};
use super::CommandResult;

/// Removes a todo by its ID
pub fn remove_todo(id: u64) -> CommandResult {
    let mut todos = load_todos()?;
    let original_len = todos.len();
    
    todos.retain(|t| t.id != id);
    
    if todos.len() < original_len {
        save_todos(&todos)?;
        println!("Removed todo #{}", id);
        
        // Show the updated list
        display_todos(&todos);
        Ok(())
    } else {
        Err(anyhow::anyhow!("Todo #{} not found", id))
    }
}
