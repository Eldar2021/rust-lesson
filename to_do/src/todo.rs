pub struct Todo {
    pub name: String,
    pub done: bool,
}

impl Todo {
    pub fn new(name: String) -> Self {
        Todo { name, done: false }
    }

    pub fn mark_done(&mut self) {
        self.done = true;
    }

    pub fn print(&self, index: usize) {
        if self.done {
            println!("{}: {} ✅", index, self.name);
        } else {
            println!("{}: {}", index, self.name);
        }
    }
}
