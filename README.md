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
cargo run -- add "Pay rent" --due 2023-11-01
```

### List tasks

```bash
# List all todos with status and due dates
cargo run -- list

# Example output:
# ID    Status  Title                        Created        Due         
# ----- ------- ---------------------------- -------------- ------------
# 1     [ ]     Buy milk                     2h ago        -           
# 2     [ ]     Buy groceries                1h ago        Tomorrow    
# 3     [ ]     Pay rent                     30m ago       Nov 01      
```

### Mark a task as done

```bash
cargo run -- done 1
```

### Remove a task

```bash
cargo run -- remove 1
```

### Clear all tasks

```bash
cargo run -- clear
```

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

