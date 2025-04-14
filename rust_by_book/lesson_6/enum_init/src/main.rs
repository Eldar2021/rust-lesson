fn main() {
    enum1();
    enum2();
    enum3();
}

// 1
enum ApiAddresFind {
    V4,
    V6,
}

fn route(ip_kind: ApiAddresFind) -> String {
    match ip_kind {
        ApiAddresFind::V4 => String::from("http/api/v4/"),
        ApiAddresFind::V6 => String::from("http/api/v6/"),
    }
}

fn enum1() {
    let four = ApiAddresFind::V4;
    let six = ApiAddresFind::V6;
    println!("four address: {}", route(four));
    println!("six address: {}", route(six));
}

// 2
enum ApiAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn enum2() {
    let four2 = ApiAddr::V4(127, 0, 0, 1);
    let six2 = ApiAddr::V6(String::from("http/127.0.0.1"));

    match four2 {
        ApiAddr::V4(a, b, c, d) => println!("four2 address: {} {} {} {}", a, b, c, d),
        ApiAddr::V6(s) => println!("six2 address: {}", s),
    };

    match six2 {
        ApiAddr::V4(a, b, c, d) => println!("four2 address: {} {} {} {}", a, b, c, d),
        ApiAddr::V6(s) => println!("six2 address: {}", s),
    };
}

// 3
enum Option<T> {
    None,
    Some(T),
}

fn enum3() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;

    match sum {
        Option::Some(s) => println!("sum: {}", s),
        Option::None => println!("sum: none"),
    }
}
