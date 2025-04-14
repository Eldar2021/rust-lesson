fn main() {
    fn_1();
    fn_2();
    fn_3();
    fn_4();
    fn_5();
}

/// Dereferencing a Raw Pointer
fn fn_1() {
    let mut a = 5;

    let b = &a as *const i32;
    let c = &mut a as *mut i32;

    // println!("B value {}", *b);
    // println!("C value {}", *c);

    unsafe {
        println!("B value {}", *b);
        println!("C value {}", *c);
    }
}

/// Calling an Unsafe Function or Method
unsafe fn dangerous() {
    println!("this is Dangerous dunction")
}
fn fn_2() {
    // dangerous();
    unsafe {
        dangerous();
    }
}

/// Creating a Safe Abstraction over Unsafe Code
fn fn_3() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    println!("A: {a:?}");
    println!("B: {b:?}");

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6, 7, 8, 9]);
}

// Using extern Functions to Call External Code
extern "C" {
    fn abs(input: i32) -> i32;
}
/// Using extern Functions to Call External Code
fn fn_4() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

/// Accessing or Modifying a Mutable Static Variable
static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: i32 = 12;

fn add_counter(inc: i32) {
    unsafe {
        COUNTER += inc;
    }
}

fn fn_5() {
    println!("name is: {HELLO_WORLD}");

    // println!("counter is: {COUNTER}");

    add_counter(12);

    // unsafe {
    //     println!("counter is: {:?}", COUNTER);
    // };

    add_counter(12);
}
