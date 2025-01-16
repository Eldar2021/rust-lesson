fn main() {
    enum1();
}

enum Option<T> {
    None,
    Some(T),
}

fn enum1() {
    let config_max: Option<u8> = Option::Some(3);

    match config_max {
        Option::Some(max) => println!("The maximum is configured to be 1 {}", max),
        _ => (),
    }

    if let Option::Some(max) = config_max {
        println!("The maximum is configured to be 2 {}", max);
    }

    // with else
    let dice_roll: Option<u8> = Option::None;
    if let Option::Some(max) = dice_roll {
        println!("The maximum is configured to be 3 {}", max);
    } else {
        println!("The maximum is not configured");
    }
}
