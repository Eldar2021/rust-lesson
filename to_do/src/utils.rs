use std::io::{stdin, stdout, Write};

pub const MENU_TEXT: &str = "
-------------Init-------------------------
1. Add Todo
2. View Todos
3. Mark Todo as Done
4. Delete Todo
5. Exit
";

pub fn input(prompt: &str) -> String {
    print!("{prompt}");
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn get_index(input: &str, max: usize) -> Option<usize> {
    match input.trim().parse::<usize>() {
        Ok(i) if i > 0 && i <= max => Some(i - 1),
        _ => None,
    }
}
