fn main() {
    fn_1();
    fn_2();
    fn_3();
    fn_4();
    fn_5();
    fn_6();
    fn_7();
    fn_8();
    fn_9();
    fn_10();
}

/// Matching Literals
fn fn_1() {
    let x = 5;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

/// Matching Named Variables
fn fn_2() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(z) => println!("Matched, z = {z}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");
}

/// Multiple Patterns
fn fn_3() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

/// Matching Ranges of Values with ..=
fn fn_4() {
    let x = 3;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let y = 'c';

    match y {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

/// Destructuring Structs
struct Point {
    x: i32,
    y: i32,
}

fn fn_5() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 1, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}

/// Destructuring Enums
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn fn_6() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}")
        }
    }
}

/// Ignoring Values in a Pattern
fn fn_7() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _second, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }
}

/// Ignoring Remaining Parts of a Value with
#[allow(dead_code)]
struct PointSecond {
    x: i32,
    y: i32,
    z: i32,
}

fn fn_8() {
    let origin = PointSecond { x: 0, y: 0, z: 0 };

    match origin {
        PointSecond { x, .. } => println!("x is {x}"),
    }
}

/// Extra Conditionals with Match Guards
fn fn_9() {
    let num = Some(5);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }
}

/// @ Bindings ile Kullanım
fn fn_10() {
    let number = Some(12);

    match number {
        Some(x @ 10..15) => println!("Value is bigger than 10 smaller than 15 value is: {}", x),
        Some(x) => println!("Value is: {}", x),

        None => println!("Value None"),
    }
}
