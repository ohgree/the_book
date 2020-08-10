pub fn add_two(a: i32) -> i32 {
    a + 2
}
pub fn greeting(name: &str) -> String {
    format!("Hello {{!")
}
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 {
            panic!("Guess must be >= 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess must be <= 100, got {}.", value);
        }
        Guess { value }
    }
}
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got {}", a);
    10
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn expensive_test() {}

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }

    #[test]
    //fn this_test_will_fail() {
    //let value = prints_and_returns_10(4);
    //assert_eq!(value, 5);
    //}
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err("Error!".to_owned())
        }
    }

    #[test]
    #[should_panic(expected = "Guess must be <= 100, got ")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic]
    fn less_than_1() {
        Guess::new(-1);
    }
    //#[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeing did not contain name, value: {}",
            result
        );
    }
    //#[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
    //#[test]
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

    //#[test]
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
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}
