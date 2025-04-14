use std::time::Instant;

fn main() {
    let numbers: Vec<i32> = (1..=65535).collect();

    // Loops
    let start_loop = Instant::now();
    let mut sum_loop: i32 = 0;
    for i in &numbers {
        sum_loop += i
    }
    let duration_loop = start_loop.elapsed();

    // Iteration
    let start_iter = Instant::now();
    let sum_iter: i32 = numbers.iter().sum();
    let duration_iter = start_iter.elapsed();

    // Printing
    println!("Loop sum: {} (Duration: {:?})", sum_loop, duration_loop);
    println!("Iterator sum: {} (Duration: {:?})", sum_iter, duration_iter);
}
