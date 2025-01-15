fn main() {
    println!("Hello, world!");
    exmaple_with_function();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn exmaple_with_function() {
    // v1
    let width = 10;
    let height = 20;
    println!("Area v1: {}", calculate_area_v1(width, height));

    // v2
    let rect1 = (10, 20);
    println!("Area v2: {}", calculate_area_v2(rect1));

    // v3
    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };
    println!("Area v3: {}", calculate_area_v3(rect2));
    let rect3 = Rectangle {
        width: 10,
        height: 20,
    };
    println!("rect3 is {rect3:?}");
    println!("rect3 is {rect3:#?}");
}

fn calculate_area_v1(width: u32, height: u32) -> u32 {
    width * height
}

fn calculate_area_v2(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

fn calculate_area_v3(rect: Rectangle) -> u32 {
    rect.width * rect.height
}
