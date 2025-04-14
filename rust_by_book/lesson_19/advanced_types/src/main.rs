use core::fmt;
use std::fmt::Error;

/// Advanced Types
fn main() {
    fn_1();
    fn_2();
    fn_3();
    fn_4();
}

/// Using the Newtype Pattern for Type Safety and Abstraction
pub trait Write1 {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}

type ResultT<T> = std::result::Result<T, std::io::Error>;

pub trait Write2 {
    fn write(&mut self, buf: &[u8]) -> ResultT<usize>;
    fn flush(&mut self) -> ResultT<()>;

    fn write_all(&mut self, buf: &[u8]) -> ResultT<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> ResultT<()>;
}

fn fn_1() {
    println!("trait Write1 == Write2")
}

/// The Never Type that Never Returns
// fn bar() -> ! {
//     panic!("No data")
// }
fn fn_2() {
    // bar();
}

///
fn fn_3() {
    let s1 = "Hello there!";
    // let s2: str = "How's it going?"; error

    println!("{}", s1);
}

/// Dynamically Sized Types and the Sized Trait
fn generic_function<T>(_t: T) {
    // T, varsayılan olarak Sized'tir
    // generic_function, T türünün Sized olmasını gerektirir.
}

fn generic_function_dst<T: ?Sized>(_t: &T) {
    // T, Sized olmayabilir (DST olabilir)
}
fn fn_4() {
    generic_function(21);
    generic_function_dst(&21);
}
