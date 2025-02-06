/// Match Arms
///
/// As discussed in Chapter 6, we use patterns in the arms of match expressions.
/// Formally, match expressions are defined as the keyword match, a value to match on,
/// and one or more match arms that consist of a pattern and an expression
/// to run if the value matches that arm’s pattern, like this:
/// ```rust
/// match VALUE {
///     PATTERN => EXPRESSION,
///     PATTERN => EXPRESSION,
///     PATTERN => EXPRESSION,
/// }
/// ```
fn main() {
    fn_1();
    fn_2();
    fn_3();
    fn_4();
}

/// Match example
fn fn_1() {
    let v = Some(5);

    match v {
        Some(x) => println!("Value is {}", x),
        None => println!("Value is None"),
    }
}

/// If let, else if, else if let
fn fn_2() {
    let fav_color: Option<&str> = None;
    let is_tuesday: bool = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = fav_color {
        println!("Fav color is {}", color);
    } else if is_tuesday {
        println!("Today is Tuesday");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Age bigger than 30, age value is: {}", age)
        } else {
            println!("Age smaller than 30, age value is: {}", age)
        }
    } else {
        println!("All conditions is not true");
    }
}

/// While let Conditional Loops
fn fn_3() {
    let mut stack: Vec<i32> = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(x) = stack.pop() {
        println!("Curent value is: {}", x)
    }
}

/// For Loops
fn fn_4() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }
}
