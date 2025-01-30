use add_one::add_one;
use add_two::add_two;

fn main() {
    println!("Hello, world!");

    let a = 10;
    let b = add_one(a);
    println!("Result b: {}", b);

    let c = 20;
    let d = add_two(c);
    println!("Result d: {}", d);
}
