use std::thread;

fn main() {
    fn_1();
    fn_2();
}

// 1) using another tread
fn fn_1() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the new thread", i);
        }
    });

    handle.join().unwrap();

    for i in 1..10 {
        println!("Hello number {} from the main thread", i);
    }
}

// 2) Using move for shared data referance
fn fn_2() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
