/// Advanced Functions and Closures
fn main() {
    fn_1();
    fn_2();
}

/// Function Pointers
fn add(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn fn_1() {
    let res = do_twice(add, 10);
    println!("Result: {}", res);
}

/// Returning Closures
fn return_calousure() -> fn(i32) -> i32 {
    |x| x + 1
}

fn return_calousure_2(f: fn(i32) -> i32) -> fn(i32) -> i32 {
    f
}

fn return_calousure_3() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn remove(x: i32) -> i32 {
    x - 1
}
fn fn_2() {
    let function = return_calousure();

    let function_2 = return_calousure_2(remove);

    let function_3 = return_calousure_3();

    println!("Res 1: {}", function(3));
    println!("Res 2: {}", function_2(3));
    println!("Res 3: {}", function_3(4));
}
