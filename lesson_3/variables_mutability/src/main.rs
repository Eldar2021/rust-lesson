fn main() {
    // Variables are immutable by default
    let x= 5;
    println!("The value of x is (1): {}", x);
    // x = 6; // Cannot assign to `x` because it is not mutable
    // println!("The value of x is (2): {}", x);

    // Variables can be mutable
    let mut y = 5;
    println!("The value of y is (1): {}", y);
    y = 6;
    println!("The value of y is (2): {}", y);

    // Constants are immutable by default
    const THREE_HOURS_IN_SECONDS :u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is (1): {}", THREE_HOURS_IN_SECONDS);

    // Shadowing
    let a = 5;
    println!("The value of a is (1): {}", a);
    let a = a + 1;
    println!("The value of a is (2): {}", a);

    // Inner scope
    let b = 5;
    println!("The value of b is (1): {}", b);
    {
        let b = 6;
        println!("The value of b is (2): {}", b);
    }
    println!("The value of b is (3): {}", b);

    // Change types
    let c = "abc";
    println!("The value of c is (1): {}", c);
    let c = c.len();
    println!("The value of c is (2): {}", c);

    // Can not change types when use mut
    // let mut d = "abc"; // Cannot assign to `d` because it is not mutable
    // println!("The value of d is (1): {}", d);
    // d = d.len(); // Cannot assign to `d` because it is not mutable
    // println!("The value of d is (1): {}", d);
}
