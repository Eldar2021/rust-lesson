fn main() {
    enum1();
    enum2();
}

#[derive(Debug)]
enum UsState {
    NewYork,
    California,
}

// 1
enum Coin {
    Penny,
    Nickel,
    Dime(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime(state) => {
            println!("Dime in {state:#?}");
            10
        }
    }
}

fn enum1() {
    let a1 = Coin::Penny;
    let v1 = value_in_cents(a1);
    println!("v1: {}", v1);

    let a2 = Coin::Nickel;
    let v2 = value_in_cents(a2);
    println!("v2: {}", v2);

    let a3 = Coin::Dime(UsState::NewYork);
    let v3 = value_in_cents(a3);
    println!("v3: {}", v3);

    let a4 = Coin::Dime(UsState::California);
    let v4 = value_in_cents(a4);
    println!("v4: {}", v4);
}

// 2
#[derive(Debug)]
enum Option<T> {
    None,
    Some(T),
}

fn plus_one(x: Option<u32>) -> Option<u32> {
    match x {
        Option::None => Option::None,
        Option::Some(i) => Option::Some(i + 1),
    }
}

fn enum2() {
    let a1: Option<u32> = Option::None;
    let a2: Option<u32> = Option::Some(1);

    println!("a1: {a1:#?}");
    println!("a2: {a2:#?}");

    let b1 = plus_one(a1);
    let b2 = plus_one(a2);

    println!("b1: {b1:#?}");
    println!("b2: {b2:#?}");

    let c1 = 7;

    match c1 {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    };

    println!("-----------------------");

    let d1 = 5;

    match d1 {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => (),
    };
}
