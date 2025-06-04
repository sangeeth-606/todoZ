use std::io::{self, Write};

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
        let checkbox = if self.completed { "[x]" } else { "[ ]" };
        format!("{} {}: {}", self.id, checkbox, self.description)
    }
}

fn add_task(tasks: &mut Vec<Task>, description: String) {
    let id = (tasks.len() + 1) as u32;
    tasks.push(Task::new(id, description));
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

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    println!("Welcome to the To-Do App! (Type 'list', 'add <task>', '+ <task>', or 'quit')");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        if input == "quit" {
            break;
        }

        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        match parts[0] {
            "list" => {
                list_tasks(&tasks);
            }
            "add" | "+" => {
                if parts.len() < 2 || parts[1].is_empty() {
                    println!("Please provide a task description.");
                } else {
                    add_task(&mut tasks, parts[1].to_string());
                    println!("Task added!");
                    list_tasks(&tasks);
                }
            }
            _ => println!("Unknown command. Use 'list', 'add <task>', '+ <task>', or 'quit'."),
        }
    }
}