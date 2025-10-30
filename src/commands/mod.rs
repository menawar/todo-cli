//! Command handlers for the todo application

mod add;
mod clear;
mod done;
mod list;
mod priority;
mod remove;

pub use add::add_todo;
pub use clear::clear_todos;
pub use done::mark_done;
pub use list::list_todos;
pub use priority::set_priority;
pub use remove::remove_todo;

use anyhow::Result;

/// Common result type for command handlers
pub type CommandResult = Result<()>;
