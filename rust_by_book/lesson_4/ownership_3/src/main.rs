fn main() {
    let s = String::from("hello");
    let slice1 = &s[0..2];
    let slice2 = &s[2..];

    println!("1 {}", slice1);
    println!("2 {}", slice2);
}
