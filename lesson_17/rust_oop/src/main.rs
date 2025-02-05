use rust_oop::{self, AveragedCollection};

fn main() {
    println!("Hello, world!");
    let mut v1 = AveragedCollection {
        list: vec![122.1, 23.34, 343.34],
        average: 0.0,
    };

    v1.add(64.6);

    println!("Average: {}", v1.average);
}
