fn main() {
    let guess1: u32 = "42".parse().expect("Not a number!");

    println!("The value of guess1 is: {}", guess1);

    // Scaler types
    // integers, float, Booleans, characters
    let _x: u8 = 5;
    let _y: u16 = 5;
    let _z: u32 = 5;
    let _w: u64 = 5;
    let _v: u128 = 5;
    let _u: usize = 5;

    let _tup = (500, 6.4, 1);

    let (_x, y, _z) = _tup;

    println!("The value of y is: {y}");

    let _x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = _x.0;

    let _six_point_four = _x.1;

    let _one = _x.2;

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("a is a list {:?}", a);
}
