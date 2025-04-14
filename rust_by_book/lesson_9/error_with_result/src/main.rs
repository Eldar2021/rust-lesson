use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    function1();
    function2();
    // function3();
    // function4();

    let r1 = function5();

    match r1 {
        Ok(r2) => println!("r2: {}", r2),
        Err(e) => println!("Error ocurred function5: {}", e),
    }

    let r3 = function6();

    match r3 {
        Ok(r4) => println!("r4: {}", r4),
        Err(e) => println!("Error ocurred function6: {}", e),
    }
}

fn function1() {
    let file_result = File::open("hello.txt");

    let file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    println!("{:?}", file);
}

fn function2() {
    let file = File::open("hello_1.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello_1.txt").unwrap_or_else(|error| {
                panic!("Error ocuured while creating the file: {:?}", error);
            })
        } else {
            panic!("Error ocuured while opening the file: {:?}", error);
        }
    });

    println!("{:?}", file);
}

fn function3() {
    let file = File::open("hello_2.txt").unwrap();
    println!("{:?}", file);
}

fn function4() {
    let file = File::open("hello_3.txt").expect("Error ocurred while opening the file");
    println!("{:?}", file);
}

fn function5() -> Result<String, std::io::Error> {
    let file_result = File::open("hello_4.txt");

    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut user_name = String::new();

    match file.read_to_string(&mut user_name) {
        Ok(_) => Ok(user_name),
        Err(e) => Err(e),
    }
}

fn function6() -> Result<String, std::io::Error> {
    fs::read_to_string("hello_4.txt")
}
