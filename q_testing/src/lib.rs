#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn _can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::{add_two, Rectangle};

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn exploration() {
        assert_ne!(2 + 2, 5);
    }

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

        assert!(larger._can_hold(&smaller));
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

        assert!(!smaller._can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = super::greeting("Carol");
        assert!(
            result.contains("Hello!"),
            "Greeting did not contain name, the value was {}",
            result
        );
    }

    pub struct Guess {
        #[allow(dead_code)]
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 {
                panic!(
                    "Guess value must be greater than or equal to 1, got {}.",
                    value
                );
            } else if value > 100 {
                panic!(
                    "Guess value must be less than or equal to 100, got {}.",
                    value
                );
            }
            Guess { value }
        }
    }

    // substring in should_panic will test if the panic message contains the substring
    #[test]
    #[should_panic = "Guess value must be less than or equal to 100"]
    fn greater_than_100() {
        Guess::new(200);
    }

    // -------- USING RESULTS -----------
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 3 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    fn prints_and_returns_10(a: i32) -> i32 {
        println!("I got the value {}", a);
        10
    }

    // Run with cargo test -- --show-output
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    // cargo run add will run both the next two tests
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    // to run only the test below: `cargo test -- --ignored`
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
        assert!(true);
    }
}
