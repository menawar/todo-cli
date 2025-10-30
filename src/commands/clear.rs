use std::io::{self, Write};
use crate::{
    storage::save_todos,
    display::display_todos,
};
use super::CommandResult;

/// Clears all todos after confirmation
pub fn clear_todos() -> CommandResult {
    // Ask for confirmation
    print!("Are you sure you want to clear all todos? (y/N): ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    if input.trim().eq_ignore_ascii_case("y") {
        save_todos(&[])?;
        println!("All todos have been cleared.");
    } else {
        println!("Operation cancelled.");
    }
    
    // Show empty list
    display_todos(&[]);
    
    Ok(())
}
