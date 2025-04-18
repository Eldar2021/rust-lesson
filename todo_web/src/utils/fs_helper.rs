use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Write},
    sync::Mutex,
};

use once_cell::sync::Lazy;

use crate::models::{Todo, TodoCreationParam, TodoEditionParam};

static FILE_PATH: &str = "todo.json";

pub static FILE_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

pub fn read(query: Option<HashMap<String, String>>) -> Vec<Todo> {
    let todos = get();

    match query {
        Some(param) => match param.get("completed") {
            Some(value) => filter_todos(todos, value),
            None => todos,
        },
        _ => todos,
    }
}

pub fn write(param: TodoCreationParam) {
    let _lock = FILE_MUTEX.lock().unwrap();

    let mut todos = get();
    let id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;

    todos.push(Todo::create_from_param(param, id));
    let json = serde_json::to_string_pretty(&todos).unwrap();

    let mut file = File::create(FILE_PATH).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

pub fn update(id: i32, param: TodoEditionParam) {
    let _lock = FILE_MUTEX.lock().unwrap();

    let mut todos = get();

    if let Some(item) = todos.iter_mut().find(|i| i.id == id) {
        item.update_from_param(param);
        let json = serde_json::to_string_pretty(&todos).unwrap();

        let mut file = File::create(FILE_PATH).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    } else {
        eprintln!("Todo with {} id not found", id);
    }
}

pub fn delete(id: i32) {
    let _lock = FILE_MUTEX.lock().unwrap();

    let mut todos = get();

    if let Some(i) = todos.iter_mut().position(|i| i.id == id) {
        todos.remove(i);
        let json = serde_json::to_string_pretty(&todos).unwrap();

        let mut file = File::create(FILE_PATH).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    } else {
        eprintln!("Todo with {} id not found", id);
    }
}

fn get() -> Vec<Todo> {
    match File::open(FILE_PATH) {
        Ok(file) => {
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
        }
        Err(_) => Vec::new(),
    }
}

fn filter_todos(todos: Vec<Todo>, filter: &str) -> Vec<Todo> {
    if filter == "true" {
        todos.into_iter().filter(|e| e.completed).collect()
    } else if filter == "false" {
        todos.into_iter().filter(|e| !e.completed).collect()
    } else {
        todos
    }
}
