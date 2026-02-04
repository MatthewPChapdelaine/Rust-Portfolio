/*
 * TODO CLI - Simple task manager
 * Compile: rustc todo-cli.rs
 * Run: ./todo-cli
 */

use std::fs;
use std::io::{self, Write};

#[derive(Debug)]
struct Todo {
    task: String,
    done: bool,
}

const TODO_FILE: &str = "todos.txt";

fn load_todos() -> Vec<Todo> {
    let mut todos = Vec::new();
    
    if let Ok(contents) = fs::read_to_string(TODO_FILE) {
        for line in contents.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {
                todos.push(Todo {
                    task: parts[1].to_string(),
                    done: parts[0] == "1",
                });
            }
        }
    }
    
    todos
}

fn save_todos(todos: &[Todo]) {
    let mut contents = String::new();
    for todo in todos {
        let done_flag = if todo.done { "1" } else { "0" };
        contents.push_str(&format!("{}|{}\n", done_flag, todo.task));
    }
    fs::write(TODO_FILE, contents).expect("Unable to write file");
}

fn list_todos(todos: &[Todo]) {
    if todos.is_empty() {
        println!("No tasks yet!");
        return;
    }
    
    println!("\n=== Your Tasks ===");
    for (i, todo) in todos.iter().enumerate() {
        let status = if todo.done { "X" } else { " " };
        println!("{}. [{}] {}", i + 1, status, todo.task);
    }
}

fn main() {
    let mut todos = load_todos();
    
    loop {
        println!("\n=== TODO CLI ===");
        println!("1. List tasks");
        println!("2. Add task");
        println!("3. Complete task");
        println!("4. Exit");
        print!("\nChoice: ");
        io::stdout().flush().unwrap();
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read");
        
        match choice.trim() {
            "1" => list_todos(&todos),
            "2" => {
                print!("Enter task: ");
                io::stdout().flush().unwrap();
                let mut task = String::new();
                io::stdin().read_line(&mut task).expect("Failed to read");
                todos.push(Todo {
                    task: task.trim().to_string(),
                    done: false,
                });
                save_todos(&todos);
                println!("Added: {}", task.trim());
            }
            "3" => {
                list_todos(&todos);
                print!("Task number to complete: ");
                io::stdout().flush().unwrap();
                let mut num_str = String::new();
                io::stdin().read_line(&mut num_str).expect("Failed to read");
                if let Ok(num) = num_str.trim().parse::<usize>() {
                    if num > 0 && num <= todos.len() {
                        todos[num - 1].done = true;
                        save_todos(&todos);
                        println!("Completed: {}", todos[num - 1].task);
                    } else {
                        println!("Invalid task number");
                    }
                } else {
                    println!("Invalid number");
                }
            }
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}
