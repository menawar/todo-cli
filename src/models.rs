use chrono::{DateTime, Local, NaiveDate};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, str::FromStr};

/// Represents the priority level of a todo item
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default, ValueEnum)]
pub enum Priority {
    /// Low priority task
    Low,
    /// Normal priority task (default)
    #[default]
    Normal,
    /// High priority task
    High,
    /// Urgent priority task
    Urgent,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "Low"),
            Priority::Normal => write!(f, "Normal"),
            Priority::High => write!(f, "High"),
            Priority::Urgent => write!(f, "URGENT"),
        }
    }
}

impl FromStr for Priority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" => Ok(Priority::Low),
            "normal" => Ok(Priority::Normal),
            "high" => Ok(Priority::High),
            "urgent" => Ok(Priority::Urgent),
            _ => Err(format!("Invalid priority: {}", s)),
        }
    }
}

/// Represents a date input that can be today, tomorrow, or a specific date
#[derive(Debug, Clone, Copy)]
pub enum DateInput {
    Today,
    Tomorrow,
    Date(NaiveDate),
}

/// Represents a todo item
#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Local>,
    pub due_date: Option<NaiveDate>,
    #[serde(default)]
    pub priority: Priority,
}

impl Todo {
    /// Creates a new Todo with the given parameters
    pub fn new(id: u64, title: String, due_date: Option<NaiveDate>, priority: Priority) -> Self {
        Self {
            id,
            title,
            completed: false,
            created_at: Local::now(),
            due_date,
            priority,
        }
    }
}

// Implement ordering for todos based on priority, due date, and creation time
impl PartialEq for Todo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Todo {}

impl PartialOrd for Todo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Todo {
    fn cmp(&self, other: &Self) -> Ordering {
        // First, sort by completion status (incomplete first)
        match (self.completed, other.completed) {
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            _ => {
                // Then by priority (highest first)
                match self.priority.cmp(&other.priority).reverse() {
                    Ordering::Equal => {
                        // Then by due date (earliest first, with None at the end)
                        match (self.due_date, other.due_date) {
                            (Some(a), Some(b)) => a.cmp(&b),
                            (Some(_), None) => Ordering::Less,
                            (None, Some(_)) => Ordering::Greater,
                            (None, None) => {
                                // Finally, by creation time (oldest first)
                                self.created_at.cmp(&other.created_at)
                            }
                        }
                    }
                    ordering => ordering,
                }
            }
        }
    }
}
