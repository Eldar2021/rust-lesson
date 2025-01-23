fn main() {
    println!("Hello, world!");
    fn1();
    fn2();
    fn3();
    fn4();
}

// function definition

fn largest_i32_v1(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char_v1(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn fn1() {
    let number_list = vec![34, 50, 25, 100, 65];
    let res1 = largest_i32_v1(&number_list);
    println!("largest_i32_v1 result: {}", res1);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let res2 = largest_char_v1(&char_list);
    println!("largest_char_v1 result: {}", res2);

    let res3 = largest(&number_list);
    let res4 = largest(&char_list);
    println!("largest result: {} - {}", res3, res4);
}

// struct defination

#[derive(Debug)]
#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
#[allow(dead_code)]
struct PowerPoint<T, U> {
    x: T,
    y: U,
}

fn fn2() {
    let int_point = Point { x: 5, y: 10 };
    println!("int_point: {int_point:#?}");
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("float_point: {float_point:#?}");
    let char_point = Point { x: 'a', y: 'b' };
    println!("char_point: {char_point:#?}");
    let int_power_point = PowerPoint { x: 5, y: "abc'" };
    println!("int_power_point: {int_power_point:#?}");
}

// Enum defination

#[derive(Debug)]
#[allow(dead_code)]
enum ResultEnum<T, E> {
    Ok(T),
    Err(E),
}

fn fn3() {
    let res1: ResultEnum<i32, &str> = ResultEnum::Ok(5);
    let res2: ResultEnum<i32, &str> = ResultEnum::Err("error message");
    println!("res1: {:?}", res1);
    println!("res2: {:?}", res2);
}

// Method defination

#[derive(Debug)]
struct SomeStruct<T> {
    value: T,
}

impl<T> SomeStruct<T> {
    fn get_value(&self) -> &T {
        &self.value
    }
}

fn fn4() {
    let some_struct_1 = SomeStruct { value: 5 };
    println!("some_struct_1: {some_struct_1:#?}");
    println!("some_struct_1: {}", some_struct_1.get_value());
}
