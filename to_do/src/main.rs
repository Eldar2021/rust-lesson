mod actions;
mod todo;
mod utils;

use actions::{add_todo, delete_one, mark_done, show_all};
use todo::Todo;
use utils::{input, MENU_TEXT};

fn main() {
    let mut todos: Vec<Todo> = Vec::new();

    println!("Welcome ToDo List App");

    loop {
        println!("{}", MENU_TEXT);

        let choice = input("Please select an option:");

        let selection = match choice.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Invalid input: {}", e);
                continue;
            }
        };

        match selection {
            1 => add_todo(&mut todos),
            2 => show_all(&mut todos),
            3 => mark_done(&mut todos),
            4 => delete_one(&mut todos),
            5 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid selection. Please try again.");
                continue;
            }
        }
    }
}
