use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Serialize, Deserialize)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
}

impl Task {
    fn new(id: u32, description: String) -> Task {
        Task {
            id,
            description,
            completed: false,
        }
    }

    fn display(&self) -> String {
        let (symbol, style) = if self.completed {
            ("✓", "bright_green")
        } else {
            ("◯", "bright_cyan")
        };

        
        let id_str = if self.id < 10 {
            format!("0{}", self.id).bright_black()
        } else {
            format!("{}", self.id).bright_black()
        };

        let description = if self.completed {
            format!("  {}", self.description)
                .bright_black()
                .strikethrough()
        } else {
            format!("  {}", self.description).bright_white()
        };

        format!("  {} {} {}", id_str, symbol.color(style), description)
    }
}

fn get_todo_file_path() -> Result<PathBuf, String> {
    let home_dir = dirs::home_dir().ok_or_else(|| "Could not find home directory".to_string())?;
    let todo_dir = home_dir.join(".todoz");

    fs::create_dir_all(&todo_dir)
        .map_err(|e| format!("Failed to create directory ~/.todoz: {}", e))?;
    Ok(todo_dir.join("todos.json"))
}

fn load_tasks() -> Result<Vec<Task>, String> {
    let file_path = get_todo_file_path()?;
    match fs::read_to_string(&file_path) {
        Ok(data) => {
            serde_json::from_str(&data).map_err(|e| format!("Failed to parse todos.json: {}", e))
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(format!("Failed to read todos.json: {}", e)),
    }
}

fn save_tasks(tasks: &Vec<Task>) -> Result<(), String> {
    let file_path = get_todo_file_path()?;
    let json = serde_json::to_string_pretty(tasks)
        .map_err(|e| format!("Failed to serialize tasks: {}", e))?;
    fs::write(&file_path, json).map_err(|e| format!("Failed to write to todos.json: {}", e))?;
    Ok(())
}

fn add_task(tasks: &mut Vec<Task>, description: String) -> Result<(), String> {
    let id = tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1;
    tasks.push(Task::new(id, description));
    save_tasks(tasks)
}

fn print_subtle_line() {
    println!(
        "{}",
        "  ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─".bright_black()
    );
}

fn list_tasks(tasks: &Vec<Task>) {
    println!();

    if tasks.is_empty() {
        println!(
            "{}",
            "    ✨ Your space is clear and ready"
                .bright_cyan()
                .italic()
        );
        println!(
            "{}",
            "       Add a task when inspiration strikes".bright_black()
        );
        println!();
    } else {
        let total_tasks = tasks.len();
        let completed_tasks = tasks.iter().filter(|t| t.completed).count();

        
        let progress_percentage = if total_tasks > 0 {
            (completed_tasks as f32 / total_tasks as f32 * 100.0) as u32
        } else {
            0
        };

        let progress_bar = if progress_percentage > 0 {
            let filled = (progress_percentage / 5) as usize; 
            let empty = 20 - filled;
            format!(
                "{}{}",
                "●".repeat(filled).bright_green(),
                "○".repeat(empty).bright_black()
            )
        } else {
            "○".repeat(20).bright_black().to_string()
        };

        println!(
            "{}",
            format!("    Progress: {} {}%", progress_bar, progress_percentage).bright_white()
        );

        print_subtle_line();

        for task in tasks {
            println!("{}", task.display());
        }
    }

    println!();
}

fn toggle_task(tasks: &mut Vec<Task>, id: u32) -> Result<(), String> {
    for task in tasks.iter_mut() {
        if task.id == id {
            task.completed = !task.completed;
            return save_tasks(tasks);
        }
    }
    
    let id_str = if id < 10 {
        format!("0{}", id)
    } else {
        id.to_string()
    };
    Err(format!("Task {} not found", id_str))
}

fn del_task(tasks: &mut Vec<Task>, id: u32) -> Result<(), String> {
    match tasks.iter().position(|task| task.id == id) {
        Some(index) => {
            tasks.remove(index);
            save_tasks(tasks)
        }
        None => {
            
            let id_str = if id < 10 {
                format!("0{}", id)
            } else {
                id.to_string()
            };
            Err(format!("Task {} not found", id_str))
        }
    }
}

fn clear_all_tasks(tasks: &mut Vec<Task>) -> Result<(), String> {
    tasks.clear();
    save_tasks(tasks)
}

fn start_pomodoro() {
    println!();
    show_gentle_feedback("Starting your focused work session", "🍅", "bright_green");
    println!(
        "{}",
        "      Take a deep breath and focus on one task"
            .bright_black()
            .italic()
    );
    print_subtle_line();

    let start_time = Instant::now();
    let duration = Duration::from_secs(25 * 60); 

    
    println!();
    println!(
        "{}",
        "           ╭───────────────╮           ".bright_cyan()
    );
    println!(
        "{}",
        "       ╭───┤               ├───╮       ".bright_cyan()
    );
    println!(
        "{}",
        "     ╭─┤   │               │   ├─╮     ".bright_cyan()
    );
    println!(
        "{}",
        "    │  │   │    🍅 FOCUS   │   │  │    "
            .bright_magenta()
            .bold()
    );
    println!(
        "{}{}{}",
        "    │  │   │     ".bright_cyan(),
        "25:00".bright_white().bold(),
        "     │   │  │    ".bright_cyan()
    );
    println!(
        "{}",
        "    │  │   │               │   │  │    ".bright_cyan()
    );
    println!(
        "{}",
        "     ╰─┤   │               │   ├─╯     ".bright_cyan()
    );
    println!(
        "{}",
        "       ╰───┤               ├───╯       ".bright_cyan()
    );
    println!(
        "{}",
        "           ╰───────────────╯           ".bright_cyan()
    );

    let mut last_displayed = 61; 

    loop {
        let elapsed = start_time.elapsed();
        if elapsed >= duration {
            break;
        }

        let remaining = duration - elapsed;
        let minutes = remaining.as_secs() / 60;
        let seconds = remaining.as_secs() % 60;

        
        if last_displayed != seconds {
            
            print!("\x1B[s"); 
            print!("\x1B[5A"); 

            
            let time_color = if minutes >= 20 {
                "bright_green"
            } else if minutes >= 10 {
                "bright_cyan"
            } else if minutes >= 5 {
                "bright_yellow"
            } else {
                "bright_red"
            };

            
            let colon = if seconds % 2 == 0 { ":" } else { " " };
            print!("\r");
            println!(
                "{}{}{}{}{}",
                "    │  │   │     ".bright_cyan(),
                format!("{:02}", minutes).color(time_color).bold(),
                colon.color(time_color).bold(),
                format!("{:02}", seconds).color(time_color).bold(),
                "     │   │  │    ".bright_cyan()
            );

            
            let total_seconds = 25 * 60;
            let elapsed_seconds = total_seconds - remaining.as_secs();
            let progress_percent = (elapsed_seconds as f32 / total_seconds as f32 * 100.0) as u32;

            
            let bar_width = 17;
            let filled = ((progress_percent as f32 / 100.0) * bar_width as f32) as usize;
            let empty = bar_width - filled;

            print!("\x1B[1B"); 
            print!("\r");
            println!(
                "{}{}{}{}",
                "    │  │   │ ".bright_cyan(),
                "◆".repeat(filled).color(time_color),
                "◇".repeat(empty).bright_black(),
                " │   │  │    ".bright_cyan()
            );

            
            print!("\x1B[u");
            io::stdout().flush().unwrap();

            last_displayed = seconds;
        }

        thread::sleep(Duration::from_millis(200)); 
    }

    
    print!("\x1B[8A"); 
    println!(
        "{}",
        "           ╭───────────────╮           ".bright_green()
    );
    println!(
        "{}",
        "       ╭───┤               ├───╮       ".bright_green()
    );
    println!(
        "{}",
        "     ╭─┤   │               │   ├─╮     ".bright_green()
    );
    println!(
        "{}",
        "    │  │   │  🎉 TIME'S UP! 🎉  │  │    "
            .bright_green()
            .bold()
    );
    println!(
        "{}{}{}",
        "    │  │   │     ".bright_green(),
        "00:00".bright_green().bold(),
        "     │   │  │    ".bright_green()
    );
    println!(
        "{}",
        "    │  │   │               │   │  │    ".bright_green()
    );
    println!(
        "{}",
        "     ╰─┤   │               │   ├─╯     ".bright_green()
    );
    println!(
        "{}",
        "       ╰───┤               ├───╯       ".bright_green()
    );
    println!(
        "{}",
        "           ╰───────────────╯           ".bright_green()
    );

    println!();
    show_gentle_feedback("Well done! Time for a 5-minute break", "✨", "bright_white");
    println!(
        "{}",
        "      Stretch, breathe, or take a mindful walk"
            .bright_black()
            .italic()
    );
    println!();
}

fn show_help() {
    println!();
    println!(
        "{}",
        "  ✨ Simple commands for mindful productivity:".bright_white()
    );
    println!();
    println!(
        "    {}  {:<12}  {}",
        "📋".bright_blue(),
        "list",
        "view your tasks".bright_black()
    );
    println!(
        "    {}  {:<12}  {}",
        "➕".bright_green(),
        "add",
        "create a new task".bright_black()
    );
    println!(
        "    {}  {:<12}  {}",
        "✅".bright_cyan(),
        "x",
        "toggle task completion".bright_black()
    );
    println!(
        "    {}  {:<12}  {}",
        "🗑️ ".bright_yellow(),
        "rm",
        "remove a task".bright_black()
    );
    println!(
        "    {}  {:<12}  {}",
        "🧹".bright_red(),
        "rm-all",
        "remove all tasks".bright_black()
    );
    println!(
        "    {}  {:<12}  {}",
        "🍅".bright_magenta(),
        "pom",
        "start 25-minute focus timer".bright_black()
    );
    println!(
        "    {}  {:<12}  {}",
        "❓".bright_blue(),
        "help",
        "show this guidance".bright_black()
    );
    println!(
        "    {}  {:<12}  {}",
        "👋".bright_magenta(),
        "quit",
        "exit peacefully".bright_black()
    );
    println!();
    print_subtle_line();
    println!();
}

fn show_welcome() {
    print!("\x1B[2J\x1B[1;1H");

    println!();
    println!();
    println!(
        "{}",
        "    ╭───────────────────────────────────────────╮".bright_black()
    );
    println!(
        "{}",
        "    │                                           │".bright_black()
    );
    println!(
        "{}",
        "    │             ✨  todoz  ✨               │".bright_cyan()
    );
    println!(
        "{}",
        "    │                                           │".bright_black()
    );
    println!(
        "{}",
        "    │        mindful task management            │".bright_white()
    );
    println!(
        "{}",
        "    │                                           │".bright_black()
    );
    println!(
        "{}",
        "    ╰───────────────────────────────────────────╯".bright_black()
    );
    println!();
    println!(
        "{}",
        "      Begin with 'list' to see your tasks 📋".bright_black()
    );
    println!(
        "{}",
        "      or 'help' for gentle guidance ❓".bright_black()
    );
    println!();
}

fn get_prompt() -> String {
    format!("{} ", "todoz ›".bright_cyan())
}

fn show_gentle_feedback(message: &str, emoji: &str, color: &str) {
    println!("{}", format!("    {} {}", emoji, message).color(color));
}

fn main() {
    show_welcome();

    let tasks = match load_tasks() {
        Ok(tasks) => tasks,
        Err(e) => {
            show_gentle_feedback(
                &format!("Unable to load tasks: {}", e),
                "⚠️",
                "bright_yellow",
            );
            Vec::new()
        }
    };
    let mut tasks: Vec<Task> = tasks;

    loop {
        print!("{}", get_prompt());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim();

        if input == "quit" {
            println!();
            show_gentle_feedback("Thank you for staying organized ✨", "👋", "bright_green");
            println!(
                "{}",
                "      Until next time, stay mindful"
                    .bright_black()
                    .italic()
            );
            println!();
            break;
        }

        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        match parts[0] {
            "help" => {
                show_help();
            }
            "list" | "" => {
                list_tasks(&tasks);
            }
            "add" => {
                if parts.len() < 2 || parts[1].is_empty() {
                    show_gentle_feedback("Please describe your task", "💭", "bright_black");
                } else {
                    match add_task(&mut tasks, parts[1].to_string()) {
                        Ok(_) => {
                            show_gentle_feedback("Task added successfully", "✨", "bright_green");
                            list_tasks(&tasks);
                        }
                        Err(e) => show_gentle_feedback(&e, "⚠️", "bright_red"),
                    }
                }
            }
            "x" => {
                if parts.len() < 2 || parts[1].is_empty() {
                    show_gentle_feedback(
                        "Which task? (provide the task number)",
                        "🤔",
                        "bright_black",
                    );
                } else {
                    match parts[1].parse::<u32>() {
                        Ok(id) => match toggle_task(&mut tasks, id) {
                            Ok(_) => {
                                
                                let id_str = if id < 10 {
                                    format!("0{}", id)
                                } else {
                                    id.to_string()
                                };
                                show_gentle_feedback(
                                    &format!("Task {} updated", id_str),
                                    "✅",
                                    "bright_green",
                                );
                                list_tasks(&tasks);
                            }
                            Err(e) => show_gentle_feedback(&e, "⚠️", "bright_red"),
                        },
                        Err(_) => show_gentle_feedback(
                            "Please provide a valid task number",
                            "💭",
                            "bright_black",
                        ),
                    }
                }
            }
            "rm" => {
                if parts.len() < 2 || parts[1].is_empty() {
                    show_gentle_feedback(
                        "Which task to remove? (provide the task number)",
                        "🤔",
                        "bright_black",
                    );
                } else {
                    match parts[1].parse::<u32>() {
                        Ok(id) => match del_task(&mut tasks, id) {
                            Ok(_) => {
                                
                                let id_str = if id < 10 {
                                    format!("0{}", id)
                                } else {
                                    id.to_string()
                                };
                                show_gentle_feedback(
                                    &format!("Task {} removed", id_str),
                                    "🗑️",
                                    "bright_green",
                                );
                                list_tasks(&tasks);
                            }
                            Err(e) => show_gentle_feedback(&e, "⚠️", "bright_red"),
                        },
                        Err(_) => show_gentle_feedback(
                            "Please provide a valid task number",
                            "💭",
                            "bright_black",
                        ),
                    }
                }
            }
            "rm-all" => {
                print!(
                    "{}",
                    "    🤔 Remove all tasks? This cannot be undone (y/n): ".bright_yellow()
                );
                io::stdout().flush().unwrap();
                let mut confirmation = String::new();
                io::stdin()
                    .read_line(&mut confirmation)
                    .expect("Failed to read input");
                if confirmation.trim().to_lowercase() == "y" {
                    match clear_all_tasks(&mut tasks) {
                        Ok(_) => show_gentle_feedback(
                            "All tasks cleared - fresh start!",
                            "🧹",
                            "bright_green",
                        ),
                        Err(e) => show_gentle_feedback(&e, "⚠️", "bright_red"),
                    }
                } else {
                    show_gentle_feedback("No changes made", "✋", "bright_blue");
                }
            }
            "pom" => {
                start_pomodoro();
            }
            _ => {
                show_gentle_feedback(
                    &format!("'{}' is not recognized. Try 'help' for guidance", parts[0]),
                    "💭",
                    "bright_black",
                );
            }
        }
    }
}
