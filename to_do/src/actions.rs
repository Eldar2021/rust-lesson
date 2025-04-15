use crate::todo::Todo;
use crate::utils::{get_index, input};

pub fn delete_one(todos: &mut Vec<Todo>) {
    println!("----------------------------------------");

    let index = input("Enter the index of the todo to delete:");

    let index = match get_index(&index.to_string(), todos.len()) {
        Some(i) => i,
        None => {
            println!("Invalid index. Please try again.");
            return;
        }
    };

    if let Some(e) = todos.get(index - 1) {
        println!("Deleting '{}'...", e.name);
        todos.remove(index - 1);
    }
}

pub fn mark_done(todos: &mut Vec<Todo>) {
    println!("----------------------------------------");

    let index = input("Enter the index of the todo to mark as done:");

    let index = match get_index(&index.to_string(), todos.len()) {
        Some(i) => i,
        None => {
            println!("Invalid index. Please try again.");
            return;
        }
    };

    if let Some(e) = todos.get_mut(index) {
        println!("Marking '{}' as done...", e.name);
        e.mark_done();
    }
}

pub fn show_all(todos: &Vec<Todo>) {
    println!("----------------------------------------");
    println!("Current Todos:");

    if todos.is_empty() {
        println!("No todos found.");
    } else {
        for (index, todo) in todos.iter().enumerate() {
            todo.print(index + 1);
        }
    }
}

pub fn add_todo(todos: &mut Vec<Todo>) {
    println!("----------------------------------------");

    let todo = input("Enter a new todo:");

    println!("Adding a new todo...");

    todos.push(Todo::new(todo.trim().to_string()));
}
