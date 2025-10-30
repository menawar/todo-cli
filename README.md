# Rust CLI Todo App — step-by-step

This project is a **command-line To-Do list app in Rust**. It stores tasks locally in a JSON file (`todos.json`) and provides commands to **add**, **list**, **mark as done**, **remove**, and **clear** tasks.

---

## 🦀 Features

* Add new tasks with a title
* List all tasks with their status
* Mark tasks as done
* Remove tasks by ID
* Clear all tasks at once
* Data persistence using JSON (`todos.json`)

---

## 🚀 Getting Started

### Prerequisites

* Rust toolchain (`rustup`, `cargo`): [Install Rust](https://rustup.rs)

### Clone and Build

```bash
git clone <your-repo-url>
cd todo-cli
cargo build --release
```

---

## 📦 Usage

Run the app using cargo or the compiled binary.

### Add a task

```bash
# Basic usage
cargo run -- add "Buy milk"

# With due date (today/tomorrow or YYYY-MM-DD)
cargo run -- add "Buy groceries" --due tomorrow
cargo run -- add "Pay rent" --due 2025-11-01

# With priority (low, normal, high, urgent)
cargo run -- add "Fix critical bug" --priority high
cargo run -- add "Wash car" --priority low

# With both due date and priority
cargo run -- add "Project deadline" --due 2025-11-15 --priority urgent
cargo run -- add "Weekly review" --due friday --priority normal
```

### List tasks

```bash
# List all todos with status, priority, and due dates
cargo run -- list

# Filter and sort options:
cargo run -- list --active           # Show only uncompleted tasks
cargo run -- list --priority high    # Show high/urgent priority tasks
cargo run -- list --sort due         # Sort by due date (earliest first)
cargo run -- list --sort priority    # Sort by priority (highest first)
cargo run -- list --sort created     # Sort by creation time (oldest first)

# Combined example:
cargo run -- list --active --priority high --sort due

# Example output:
# ID    Status  Priority  Title                        Created        Due         
# ----- ------- --------- ---------------------------- -------------- ------------
# 3     [ ]     URGENT    Project deadline             30m ago       Nov 15      
# 1     [ ]     High      Fix critical bug             2h ago        -           
# 5     [ ]     High      Wash car                     5m ago        in 5d       
# 2     [ ]     Normal    Buy groceries                1h ago        Tomorrow    
# 4     [✔]     Normal    Buy milk                     1d ago        -           
```

### Manage tasks

```bash
# Mark a task as done
cargo run -- done 1

# Change priority of a task
cargo run -- priority 1 high

# Remove a task
cargo run -- remove 1

# Clear all tasks (requires confirmation)
cargo run -- clear

# Example workflow:
# 1. Add a task with due date and priority
cargo run -- add "Complete project" --due 2025-11-10 --priority high

# 2. List active high-priority tasks
cargo run -- list --active --priority high

# 3. Update priority when needed
cargo run -- priority 1 urgent

# 4. Mark as done when completed
cargo run -- done 1
```

### Task Priorities
- `low`: Low priority tasks
- `normal`: Default priority
- `high`: Important tasks (shown in yellow)
- `urgent`: Critical tasks (shown in red)

### Sorting Options
- `smart`: Incomplete first, then by priority, due date, and creation time
- `due`: Sort by due date (earliest first)
- `priority`: Sort by priority (highest first)
- `created`: Sort by creation time (oldest first)

---

## ⚙️ Project Structure

```
todo-cli/
├── Cargo.toml         # Dependencies and metadata
├── src/
│   └── main.rs        # Core app logic
└── todos.json         # (Created automatically) stores todos
```

---

## 🧠 How It Works

* Tasks are stored as JSON objects with fields: `id`, `title`, `completed`.
* The app loads this list from `todos.json` on startup, modifies it in memory, and writes it back when changes occur.
* Error handling is done using the `anyhow` crate.

---

## 📚 Dependencies

```toml
clap = { version = "4", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
```

