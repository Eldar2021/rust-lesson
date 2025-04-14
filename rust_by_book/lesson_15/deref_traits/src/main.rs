use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello {}", name)
}

fn main() {
    let a = 5;
    let b = Box::new(a);
    println!("a value: {}", a);
    println!("b value: {}", *b);

    let c = 5;
    let d = MyBox::new(c);
    println!("c value: {}", c);
    println!("d value: {}", *d);

    let e = String::from("Eldiiar");
    hello(&e);
    let f = MyBox::new(String::from("Rust"));
    hello(&f);
}
