fn main() {
    println!("Hello, world!");

    some_function();

    parameter_function(10);

    parameters_function(10, 20);

    // Function body
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let x = summation(10, 20);
    println!("The value of x is: {}", x);
}

fn some_function() {
    println!("some_function ");
}

fn parameter_function(x: i32) {
    println!("parameter_function x is: {}", x);
}

fn parameters_function(x: i32, y: i32) {
    println!("parameters_function x is: {}, y is: {}", x, y);
}

fn summation(x: i32, y: i32) -> i32 {
    x + y
}