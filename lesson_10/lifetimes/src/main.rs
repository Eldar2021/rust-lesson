fn main() {
    println!("Hello, world!");
    // fn1();
    fn2();
}

// Test problem
// fn fn1() {
//     let a: i32;
//
//     {
//         let b: i32 = 10;
//         println!("b: {}", b);
//         a = &b;
//         println!("a: {}", a);
//     }
//
//     println!("a: {}", a);
// }

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn fn2() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}
