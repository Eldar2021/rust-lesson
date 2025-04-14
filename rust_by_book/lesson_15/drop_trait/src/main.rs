#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let a = CustomSmartPointer {
        data: String::from("Eldiiar"),
    };

    println!("Created 1: {}", a.data);

    let e = CustomSmartPointer {
        data: String::from("Flutter"),
    };

    println!("Created 2: {}", e.data);

    {
        let c = CustomSmartPointer {
            data: String::from("Dart"),
        };

        println!("Created 3: {}", c.data);

        drop(e);
    }

    let b = CustomSmartPointer {
        data: String::from("Rust"),
    };

    println!("Created 4: {}", b.data);
}
