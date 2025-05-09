use std::error::Error;
use std::fmt;

pub mod util {
    pub mod integer;
    pub mod parse;
}

pub mod y2015 {
    pub mod day01;
}

#[derive(Debug)]
pub struct ParseError(String);

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse puzzle input: '{}'", self.0)
    }
}
