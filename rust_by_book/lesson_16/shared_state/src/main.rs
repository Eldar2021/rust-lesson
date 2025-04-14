use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    fn_1();
    fn_2();
}

// 1) Using Mutex
fn fn_1() {
    let a = Mutex::new(1);
    println!("a1 1): {a:?}");

    {
        let mut a1 = a.lock().unwrap();
        println!("a1 2): {}", a1);
        *a1 = 2;
        println!("a1 3): {}", a1);
    }

    println!("a1 4): {a:?}");
}

// 2) Using Arc
fn fn_2() {
    let a = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..10 {
        let counter = Arc::clone(&a);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *a.lock().unwrap())
}
