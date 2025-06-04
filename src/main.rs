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
        let checkbox: &'static str = if self.completed { "[x]" } else { "[ ]" };
        format!("{} {}: {}", self.id, checkbox, self.description)
    }
}

fn main() {
    println!("Hello, world!");
}
