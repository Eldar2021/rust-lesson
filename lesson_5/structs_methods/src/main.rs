#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_bigger(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }

    fn square(size: u32) -> Rectangle {
        Self {
            width: size,
            height: size,
        }
    }

    fn increment_height(&mut self) {
        self.height = self.height + 1;
    }
}

fn main() {
    let r1 = Rectangle {
        width: 10,
        height: 20,
    };

    println!("r1: {r1:#?}");
    println!("r1 width: {}", r1.width);
    println!("r1 height: {}", r1.height);
    println!("r1 area: {}", r1.area());

    let r2 = Rectangle {
        width: 2,
        height: 9,
    };

    println!("r1 is bigger than r2: {}", r1.is_bigger(&r2));

    let r3 = Rectangle::square(17);
    println!("r3: {r3:#?}");

    let mut r4 = Rectangle {
        width: 4,
        height: 3,
    };

    println!("r4: {r4:#?}");
    r4.increment_height();
    println!("r4: {r4:#?}");
}
