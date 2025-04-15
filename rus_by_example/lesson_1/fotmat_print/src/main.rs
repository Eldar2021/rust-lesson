/// Printing is handled by a series of macros defined in std::fmt some of which are:
/// - format!: write formatted text to String
/// - print!: same as format! but the text is printed to the console (io::stdout).
/// - println!: same as print! but a newline is appended.
/// - eprint!: same as print! but the text is printed to the standard error (io::stderr).
/// - eprintln!: same as eprint! but a newline is appended.

fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("1: {} days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string.
    println!("2: {0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!(
        "3: {subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("4: Base 10:               {}", 69420); // 69420
    println!("5: Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("6: Base 8 (octal):        {:o}", 69420); // 207454
    println!("7: Base 16 (hexadecimal): {:x}", 69420); // 10f2c

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("8: {number:>5}", number = 1);

    // You can use named arguments in the format specifier by appending a `$`.
    println!("9: {number:0>width$}", number = 1, width = 5);

    // Also wen use eprint! and eprintln! to print to stderr
    // eprint!("10: This is an error message");

    // We use eprintln! to print to stderr with a newline
    eprintln!("11: This is an error message with a newline");
}
