use std::{
    sync::mpsc::{self},
    thread,
    time::Duration,
};

fn main() {
    fn_1();
    fn_2();
    fn_3();
}

// 1) Simple channel
fn fn_1() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let v1 = String::from("Hi");
        tx.send(v1).unwrap();
        // println!("Another thread v1: {}", v1);
    });

    let r1 = rx.recv().unwrap();
    println!("Received r1: {}", r1);
}

// 2) Multiple data send
fn fn_2() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let v1 = vec![
            String::from("Hi!"),
            String::from("I'm"),
            String::from("Rust"),
            String::from("Developer"),
        ];

        for i in v1 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs_f32(1.0));
        }
    });

    for i in rx {
        println!("{}", i);
    }
}

// 3) Multiple senders by using `tx.clone()`
fn fn_3() {
    let (tx1, rx) = mpsc::channel();

    let tx2 = tx1.clone();

    thread::spawn(move || {
        let v1 = vec![
            String::from("ABC"),
            String::from("DEF"),
            String::from("GHI"),
        ];

        for i in v1 {
            tx1.send(i).unwrap();
        }
    });

    thread::spawn(move || {
        let v1 = vec![
            String::from("JKL"),
            String::from("MNO"),
            String::from("PQR"),
        ];

        for i in v1 {
            tx2.send(i).unwrap();
        }
    });

    for received in rx {
        println!("Got! {}", received);
    }
}
