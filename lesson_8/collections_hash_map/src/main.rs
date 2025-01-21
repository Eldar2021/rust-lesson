use std::collections::HashMap;

fn main() {
    let mut h1 = HashMap::new();
    h1.insert("one", 1);
    h1.insert("two", 2);
    h1.insert("three", 3);
    println!("h1: {h1:#?}");
    for (key, value) in &h1 {
        println!("key: {key}, value: {value}");
    }

    let mut h2 = HashMap::new();
    let h2v1 = "Здравствуйте";
    let mut count = 0;
    for c in h2v1.chars() {
        h2.insert(c, count);
        count += 1;
    }
    println!("h2: {h2:#?}");
}
