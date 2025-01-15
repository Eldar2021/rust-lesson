struct User {
    active: bool,
    user_name: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

fn main() {
    // Struct
    let mut user_1 = User {
        active: true,
        user_name: String::from("John"),
        email: String::from("j@j.com"),
        sign_in_count: 1,
    };

    println!("Hello, my name is {}!", user_1.user_name);
    println!("Hello, my email is {}!", user_1.email);
    println!("Hello, my sign_in_count is {}!", user_1.sign_in_count);
    println!("Hello, my active is {}!", user_1.active);

    user_1.email = String::from("jhone@gmail.com");
    println!("Hi! Ivhanged my email to {}!", user_1.email);

    let user_2 = build_user(String::from("mike@gmail.com"), String::from("Mike"));

    println!("Hello, my name is {}!", user_2.user_name);
    println!("Hello, my email is {}!", user_2.email);
    println!("Hello, my sign_in_count is {}!", user_2.sign_in_count);
    println!("Hello, my active is {}!", user_2.active);

    let user_3 = User {
        active: user_2.active,
        user_name: String::from("Sam"),
        email: String::from("sma@gmail.com"),
        sign_in_count: user_2.sign_in_count,
    };

    println!("Hello, my name is {}!", user_3.user_name);

    let user_4 = User {
        email: String::from("j@j3.com"),
        ..user_3
    };

    println!("Hello, my email is {}!", user_4.email);

    // Tuple Struct
    let black = Color(1, 2, 3);

    println!("Balck first {}!", black.0);
    println!("Balck second {}!", black.1);
    println!("Balck third {}!", black.2);
    // println!("Balck fourth {}!", black.3); // Gecersiz index

    // Unit Struct
    struct AlwaysEqual;

    let subject = AlwaysEqual;
}

fn build_user(email: String, user_name: String) -> User {
    User {
        active: true,
        user_name,
        email,
        sign_in_count: 1,
    }
}
