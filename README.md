# TodoZ - A Simple Command-Line Todo Application

TodoZ is a fast, lightweight command-line todo list manager written in Rust. It helps you keep track of your tasks with a simple and intuitive interface.

![Version](https://img.shields.io/badge/version-0.1.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)

## Features

- **Create tasks** - Add tasks with descriptive text
- **Mark tasks as completed** - Toggle completion status with a simple command
- **Delete tasks** - Remove individual tasks or clear all tasks
- **Persistence** - Tasks are automatically saved to disk
- **User-friendly interface** - Simple commands and intuitive display
- **Cross-platform** - Works on Linux, macOS, and Windows

## Installation

### Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install) (1.50.0 or later)

### From Source

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/todoz.git
   cd todoz
   ```

2. Build the application:

   ```bash
   cargo build --release
   ```

3. Add to your PATH (optional):

   ```bash
   # For Linux/macOS
   cp target/release/todoz ~/.local/bin/

   # For Windows (PowerShell)
   # Copy-Item .\target\release\todoz.exe -Destination "$env:USERPROFILE\AppData\Local\Microsoft\WindowsApps"
   ```

### Using Cargo

```bash
cargo install --git https://github.com/sangeeth-606/todoz.git
```

## Usage

Start the application:

```bash
todoz
```

### Commands

| Command      | Description                   |
| ------------ | ----------------------------- |
| `list`       | Show all tasks                |
| `add <task>` | Add a new task                |
| `x <id>`     | Toggle task completion status |
| `rm <id>`    | Remove a task                 |
| `rm-all`     | Remove all tasks              |
| `help`       | Show help message             |
| `quit`       | Exit the application          |

### Examples

```
> add Buy groceries
Task added!
1 [ ]: Buy groceries

> add Finish report
Task added!
1 [ ]: Buy groceries
2 [ ]: Finish report

> x 1
Task 1 toggled!
1 [x]: Buy groceries
2 [ ]: Finish report

> rm 2
Task 2 removed!
1 [x]: Buy groceries
```

## Data Storage

TodoZ saves your tasks in `~/.todoz/todos.json` (Linux/macOS) or `%USERPROFILE%\.todoz\todos.json` (Windows).

## Build from Source

```bash
# Clone the repository
git clone https://github.com/sangeeth-606/todoZ
cd todoz

# Build
cargo build --release

# Run tests
cargo test

# Run the application
cargo run
```

## Dependencies

- [serde](https://crates.io/crates/serde) - For serializing and deserializing data
- [serde_json](https://crates.io/crates/serde_json) - For JSON handling
- [dirs](https://crates.io/crates/dirs) - For platform-specific directory paths

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Inspired by various command-line todo applications
- Built with Rust 🦀
