#![allow(unused)]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger: Rectangle = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller: Rectangle = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger: Rectangle = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller: Rectangle = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let result: String = greeting("Carol");
        assert!(
            result.contains("Carol!"),
            "Greeting did not contain name, value was {}",
            result
        );
    }

    #[test]
    #[should_panic(expected = "between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            return Ok(());
        }
        else {
            return Err(String::from("two plus two does not equal four"));
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

fn greeting(name: &str) -> String {
    return format!("Hello {}!", name);
}

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess must have a value between 1 and 100, got {}", value);
        }

        return Guess { value: value };
    }
}
