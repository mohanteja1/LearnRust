
pub mod adder {
   pub fn  add_two_numbers(x: i32, y: i32) -> i32 {
       x + y
   }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 1, 3);
    }

    #[test]
    fn exploration() {
        assert_eq!(44 - 22, 22);
    }

    #[test]
    fn another() {
        panic!("lets make it panic");
    }

    #[test]
    fn making_panic() {
        // this will panic
        assert_eq!(44-11, 22);
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }
}

#[cfg(test)]
mod rectangle_tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 44,
            height: 66,
        };
        let smaller = Rectangle {
            width: 22, 
            height: 33
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 44,
            height: 66,
        };
        let smaller = Rectangle {
            width: 22, 
            height: 33
        };
        assert!(!smaller.can_hold(&larger));
    }
}

// Testing Equality with the assert_eq! and assert_ne! Macros

pub fn add_two(num: i32) -> i32 {
    num + 2
}

#[cfg(test)]
mod some_more_tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(44), 46);
        assert_ne!(add_two(33), 33);
    }

}

// adding custom failure messages: 

use std::fmt::format;

pub fn greeting(name: &str) -> String {
    format!("hi {}", name)
}

#[cfg(test)]
mod test_greeting {
    use super::*;

    #[test]
    fn should_greet() {
        assert_eq!(greeting("mohan"), String::from("hi mohan"));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("mohan");
        assert!(
            result.contains("mohan"),
            "greeting doesnt contain the name: {}",
            result,
        );
    }
}

// checking panic using should_panic

struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(val: i32) -> Guess {
        if (val < 1) {
            panic!("guess value should be >= 1, got {}", val);
        }
        if (val > 100) {
          panic!("guess value should be <= 100, got {}", val);
        }
        Guess {
            value: val
        }
    }
}

#[cfg(test)]
mod guess_tests {
    use super::*;

    #[test]
    #[should_panic] 
    fn it_should_panic(){
        Guess::new(443);
    }

    #[test]
    #[should_panic(expected = "guess value should be >= 1")] // subset of panic message
    fn it_should_panic_withmessage() {
        Guess::new(0);
    }
}

// using Result(T, E) in tests

#[cfg(test)]
mod returning_results {

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 3 == 4 {
            Ok(())
        } else {
            Err(String::from("two + two doesn't equal to 4"))
        }
    }

    #[test]
    #[ignore = "need to work"]
    fn test_ignore() {

    }
}
