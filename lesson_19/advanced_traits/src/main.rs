use core::fmt;

fn main() {
    fn_1();
    fn_2();
    fn_3();
    fn_4();
    fn_5();
}

/// Associated Types
pub trait Iterator {
    type Item;

    fn next(&self) -> Option<Self::Item>;
}

pub struct Counter {
    index: i32,
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&self) -> Option<Self::Item> {
        Some(self.index + 1)
    }
}

pub trait IteratorGeneric<T> {
    fn next_generic(&self) -> Option<T>;
}

impl IteratorGeneric<i32> for Counter {
    fn next_generic(&self) -> Option<i32> {
        Some(self.index + 1)
    }
}

fn fn_1() {
    let c1 = Counter { index: 10 };

    let res = c1.next();

    match res {
        Some(v) => println!("Value is: {}", v),
        None => println!("Valie in None"),
    }
}

/// Default Generic Type Parameters and Operator Overloading
trait Add<Rhs = Self> {
    type Output;

    fn add(&self, rhs: Rhs) -> Self::Output;
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
struct Millimeters(u32);

#[derive(Debug, PartialEq)]
struct Meters(u32);

impl Add for Point {
    type Output = Point;

    fn add(&self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(&self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

fn fn_2() {
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = p1.add(p2);

    println!("Pint 3: {p3:?}");

    let ml1 = Millimeters(1200);
    let ml2 = Meters(2);

    let ml3 = ml1.add(ml2);

    println!("Millimeters 3: {ml3:?}")
}

/// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Uçak kullanılıyor.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Süpürgeyle uçuluyor.");
    }
}

impl Human {
    fn fly(&self) {
        println!("İnsan uçamaz!");
    }
}
fn fn_3() {
    let h1 = Human;

    h1.fly(); // İnsan uçamaz!

    Pilot::fly(&h1); // Uçak kullanılıyor.

    Wizard::fly(&h1); // Süpürgeyle uçuluyor.
}

/// Associated functions that are not methods don’t have a self parameter.
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Dog manualy fn")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Dog Animal fn")
    }
}
fn fn_4() {
    println!("fn4-1: {}", Dog::baby_name());
    println!("fn4-2: {}", <Dog as Animal>::baby_name());
}

/// Using Supertraits to Require One Trait’s Functionality Within Another Trait
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Data {
    data: String,
}

impl OutlinePrint for Data {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        std::println!("{}", "*".repeat(len + 4));
        std::println!("*{}*", " ".repeat(len + 2));
        std::println!("* {output} *");
        std::println!("*{}*", " ".repeat(len + 2));
        std::println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

fn fn_5() {
    let d1 = Data {
        data: String::from("I'm Rust developer!"),
    };

    d1.outline_print();
}
