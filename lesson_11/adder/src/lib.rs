pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn say_hello(name: &str) -> String {
    format!("Hello {}!", name)
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_success() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_failure() {
        let result = add(2, 2);
        assert_ne!(result, 5);
    }

    #[test]
    fn exploration() {
        let result = add(2, 3);
        assert_eq!(result, 5);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn say_hello_contains_name() {
        let result = say_hello("John");
        assert!(result.contains("John"), "John is not in `{result}`");
    }
}
