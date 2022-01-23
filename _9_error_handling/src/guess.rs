use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value <1 || value > 100 {
            panic!("guess value must be between 1 to 100");
        }
        Guess { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}

impl FromStr for Guess {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse::<i32>()?;
        Ok(Guess::new(value))
    }
}