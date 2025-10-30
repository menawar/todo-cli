use crate::models::*;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::Path;

const TODO_FILE: &str = "todos.json";

/// Loads todos from the JSON file, migrating legacy format if needed
pub fn load_todos() -> Result<Vec<Todo>> {
    if !Path::new(TODO_FILE).exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(TODO_FILE)
        .with_context(|| format!("Failed to read {}", TODO_FILE))?;

    // Try to parse as new format first
    if let Ok(todos) = serde_json::from_str::<Vec<Todo>>(&content) {
        return Ok(todos);
    }

    // If that fails, try to parse as legacy format
    #[derive(Deserialize)]
    struct LegacyTodo {
        id: u64,
        title: String,
        completed: bool,
    }

    let legacy_todos: Vec<LegacyTodo> = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse {}", TODO_FILE))?;

    // Convert legacy todos to new format
    let todos: Vec<Todo> = legacy_todos
        .into_iter()
        .map(|t| Todo {
            id: t.id,
            title: t.title,
            completed: t.completed,
            created_at: chrono::Local::now(),
            due_date: None,
            priority: Priority::Normal,
        })
        .collect();

    // Save the migrated todos back to the file
    save_todos(&todos)?;

    Ok(todos)
}

/// Saves todos to the JSON file
pub fn save_todos(todos: &[Todo]) -> Result<()> {
    let content = serde_json::to_string_pretty(todos)
        .with_context(|| "Failed to serialize todos")?;
    
    fs::write(TODO_FILE, content)
        .with_context(|| format!("Failed to write to {}", TODO_FILE))?;
    
    Ok(())
}
