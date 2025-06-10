use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

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
        let checkbox: &'static str = if self.completed { "[x]" } else { "[ ]" };
        format!("{} {}: {}", self.id, checkbox, self.description)
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

fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks in the list.");
    } else {
        for task in tasks {
            println!("{}", task.display());
        }
    }
}

fn toggle_task(tasks: &mut Vec<Task>, id: u32) -> Result<(), String> {
    for task in tasks.iter_mut() {
        if task.id == id {
            task.completed = !task.completed;
            return save_tasks(tasks);
        }
    }
    Err(format!("Task with ID {} not found", id))
}

fn del_task(tasks: &mut Vec<Task>, id: u32) -> Result<(), String> {
    match tasks.iter().position(|task| task.id == id) {
        Some(index) => {
            tasks.remove(index);
            save_tasks(tasks)
        }
        None => Err(format!("Task with id {} not found", id)),
    }
}

fn clear_all_tasks(tasks: &mut Vec<Task>) -> Result<(), String> {
    tasks.clear();
    save_tasks(tasks)
}

fn show_help() {
    println!("\nAvailable commands:");
    println!("  list              - Show all tasks");
    println!("  add <task>        - Add a new task");
    println!("  x <id>            - Toggle task completion status");
    println!("  rm <id>           - Remove a task");
    println!("  rm-all            - Remove all tasks");
    println!("  help              - Show this help message");
    println!("  quit              - Exit the application\n");
}

fn main() {
    let tasks = match load_tasks() {
        Ok(tasks) => tasks,
        Err(e) => {
            println!("Error loading tasks: {}", e);
            Vec::new()
        }
    };
    let mut tasks: Vec<Task> = tasks;
    println!("Welcome to the To-Do App! (Type 'help' for available commands)");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim();

        if input == "quit" {
            break;
        }

        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        match parts[0] {
            "help" => {
                show_help();
            }
            "list" => {
                list_tasks(&tasks);
            }
            "add" => {
                if parts.len() < 2 || parts[1].is_empty() {
                    println!("Please provide a task description.");
                } else {
                    match add_task(&mut tasks, parts[1].to_string()) {
                        Ok(_) => {
                            println!("Task added!");
                            list_tasks(&tasks);
                        }
                        Err(e) => println!("{}", e),
                    }
                }
            }
            "x" => {
                if parts.len() < 2 || parts[1].is_empty() {
                    println!("Please provide a task ID.");
                } else {
                    match parts[1].parse::<u32>() {
                        Ok(id) => match toggle_task(&mut tasks, id) {
                            Ok(_) => {
                                println!("Task {} toggled!", id);
                                list_tasks(&tasks);
                            }
                            Err(e) => println!("{}", e),
                        },
                        Err(_) => println!("Invalid task ID. Please provide a number."),
                    }
                }
            }
            "rm" => {
                if parts.len() < 2 || parts[1].is_empty() {
                    println!("Please provide a task Id to remove");
                } else {
                    match parts[1].parse::<u32>() {
                        Ok(id) => match del_task(&mut tasks, id) {
                            Ok(_) => {
                                println!("Task {} removed!", id);
                                list_tasks(&tasks);
                            }
                            Err(e) => println!("{}", e),
                        },
                        Err(_) => println!("Invalid task ID. Please provide a number."),
                    }
                }
            }
            "rm-all" => {
                print!("Are you sure you want to remove all tasks? (y/n): ");
                io::stdout().flush().unwrap();
                let mut confirmation = String::new();
                io::stdin()
                    .read_line(&mut confirmation)
                    .expect("Failed to read input");
                if confirmation.trim().to_lowercase() == "y" {
                    match clear_all_tasks(&mut tasks) {
                        Ok(_) => println!("All tasks have been removed."),
                        Err(e) => println!("{}", e),
                    }
                } else {
                    println!("Operation cancelled.");
                }
            }
            _ => println!("Unknown command. Type 'help' for available commands."),
        }
    }
}
