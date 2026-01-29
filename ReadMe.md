# rtask

A command-line productivity suite for managing Tasks, Habits, and Pomodoro sessions with local JSON storage.

## Features

- **Task Management**: Create, update, complete, and delete tasks
- **Habit Tracking**: Track daily/weekly habits with streak counters
- **Pomodoro Timer**: Configurable work/break intervals with session tracking
- **Local Storage**: All data persisted to JSON files (no database setup required)
- **Intuitive CLI**: Structured commands with sensible defaults

## Installation

```bash
# Clone and build
git clone <https://github.com/zacknn/cli-tool-for-task-habit-tracking-with-rust.git>
cd rtask
cargo build --release

# Optional: Add to PATH
cp target/release/rtask ~/.local/bin/
```

## Usage

### Tasks

```bash
# Create a task
rtask task enter -t "Fix Rust code" -d "Debug the CLI argument parser"

# List all tasks
rtask task read

# Mark as completed
rtask task update -i 1 --completed

# Delete a task
rtask task delete -i 1
```

### Habits

```bash
# Create a daily habit
rtask habit enter -t "Morning Run" -d "5km minimum" -f "daily"

# Check habits list
rtask habit read

# Increment streak
rtask habit update -i 1 --increment

# Update description
rtask habit update -i 1 -d "5km or 30 minutes"
```

### Pomodoro

```bash
# Start with defaults (25min work, 5min short break, 15min long)
rtask pomodoro enter

# Custom configuration
rtask pomodoro enter -w 50 -b 10 -B 20 -s 4 -l 2

# Associate with a specific task
rtask pomodoro enter -t 5

# Check current session status
rtask pomodoro read

# Stop current session
rtask pomodoro delete
```

## Storage

Data is stored locally in JSON format:

- **Location**: `rtask/data` 
- **Files**:
  - `tasks.json`
  - `habits.json`
  - `current_session.json`

### Schema Examples

**Task**:

```json
{
  "id": 1,
  "title": "Fix Rust code",
  "description": "Debug CLI parser",
  "completed": false,
  "created_at": "2026-01-30T10:00:00Z"
}
```

**Habit**:

```json
{
  "id": 1,
  "title": "Morning Run",
  "description": "5km minimum",
  "frequency": "daily",
  "streak": 5,
  "last_completed": "2026-01-29"
}
```

## Command Reference

| Entity | Command | Required Flags | Optional Flags |
|--------|---------|---------------|----------------|
| **Task** | `enter` | - | `-t`, `-d`, `-f` |
| | `delete` | `-i` | - |
| | `read` | - | - |
| | `update` | `-i` | `-t`, `-d`, `-c`, `--increment` |
| **Habit** | `enter` | - | `-t`, `-d`, `-f` |
| | `delete` | `-i` | - |
| | `read` | - | - |
| | `update` | `-i` | `-t`, `-d`, `-c`, `--increment` |
| **Pomodoro** | `enter` | - | `-w`, `-b`, `-B`, `-s`, `-l`, `-t` |
| | `delete` | - | - |
| | `read` | - | - |

## Architecture
The project follows a small, layered structure separating CLI, application logic,
domain models/services, and storage (JSON) concerns. This keeps the binary
(`src/main.rs`) focused on wiring and leaves features isolated and testable.

Example layout:

```
src/
├── main.rs               # Binary entry: initialize logging, config, CLI
├── app/                  # Application layer: command handlers & orchestration
│   └── mod.rs
├── cli/                  # CLI parsing (clap) and argument definitions
│   ├── mod.rs
│   └── args.rs
├── domain/               # Domain-level services and model aggregation
│   ├── mod.rs
│   └── models/           # Shared data structures used across features
│       ├── task.rs
│       ├── habit.rs
│       └── pomodoro.rs
├── habit/                # Habit feature: model, service, and handlers
│   ├── mod.rs
│   ├── model.rs
│   └── service.rs
├── storage/              # Persistence layer: JSON store adapter & abstractions
│   ├── mod.rs
│   └── json_store.rs
├── todo/                 # Task/todo feature: model + service + handlers
│   ├── mod.rs
│   ├── model.rs
│   └── service.rs
└── data/                 # Local runtime data directory (data/*.json files)

data/
├── tasks.json
├── habits.json
└── current_session.json
```

Design notes:
- Separation of concerns: `cli` only parses and validates arguments; `app` maps
  commands to domain services; `domain` holds shared models and pure business
  logic; `storage` implements persistence behind a small trait if desired.
- Use `serde` for (de)serialization and `dirs` (or `directories`) to locate the
  OS-specific data directory for `data/`.
- Keep feature modules (e.g., `habit`, `todo`) small and focused so tests can
  exercise services in isolation from I/O.

If you'd like, I can also update the code comments and module docs to match
this layout or implement a small `Storage` trait to decouple persistence from
business logic.

## Future Roadmap

- [ ] SQLite backend option (migrate from JSON)
- [ ] Add some ai features and try to use nlp models of api for a premodel 
- [ ] Task priorities and due dates
- [ ] Habit heatmap visualization
- [ ] Configuration file support
- [ ] Add cli interface to show some statistics

## License

MIT

```


