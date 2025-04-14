use std::{cell::RefCell, rc::Rc};

fn main() {
    immutable_exmaple();
    mutable_example();
}

fn immutable_exmaple() {
    let a = Rc::new(5);
    println!("a value: {}", a);
    println!("a reference count 1: {}", Rc::strong_count(&a));

    let b = Rc::clone(&a);
    println!("b value: {}", b);
    println!("a reference count 2: {}", Rc::strong_count(&a));

    {
        let c = Rc::clone(&a);
        println!("c value: {}", c);
        println!("a reference count 3: {}", Rc::strong_count(&a));
    }

    println!("a reference count 4: {}", Rc::strong_count(&a));
}

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    name: String,
    age: RefCell<u8>,
}

fn mutable_example() {
    let user1 = User {
        name: String::from("Eldiiar"),
        age: RefCell::new(25),
    };

    println!("user1 1: {user1:?}");

    *user1.age.borrow_mut() += 1;

    println!("user1 2: {user1:?}");
}
