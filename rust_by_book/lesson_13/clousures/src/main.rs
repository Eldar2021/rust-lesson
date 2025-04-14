#[derive(Debug, PartialEq)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("Hello, world!");

    let mut list: [Rectangle; 3] = [
        Rectangle {
            width: 4,
            height: 20,
        },
        Rectangle {
            width: 1,
            height: 20,
        },
        Rectangle {
            width: 70,
            height: 20,
        },
    ];

    let mut count: i32 = 0;

    list.sort_by_key(|r: &Rectangle| {
        count += 1;
        r.width
    });

    println!("count: {}", count);

    println!("{:?}", list);
}
