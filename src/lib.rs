#![feature(let_chains)]

use std::error::Error;
use std::iter::empty;
use std::path::{Path, PathBuf};
use std::{fmt, fs};

use crate::util::parse::ParseOps;

pub mod util {
    pub mod array;
    pub mod integer;
    pub mod parse;
}

pub mod y2015 {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
    pub mod day06;
    pub mod day07;
}

pub struct Solution {
    pub year: u32,
    pub day: u32,
    pub input_path: PathBuf,
    pub wrapper: fn(&str) -> Result<(Option<String>, Option<String>), Box<dyn Error>>,
}

macro_rules! make_solutions {
    ($year:tt $($day:tt),*) => {
        pub fn $year() -> Vec<Solution> {
            vec![$({
                let year = stringify!($year);
                let day = stringify!($day);

                let input_path = Path::new("input").join(year).join(day).with_extension("txt");

                let wrapper = |filepath: &str| {
                    use $year::$day::*;

                    let input = fs::read_to_string(filepath)?;
                    let parsed = parse(&input)?;

                    Ok((part1(&parsed).map(|v| v.to_string()), part2(&parsed).map(|v| v.to_string())))
                };

                Solution { year: year.unsigned(), day: day.unsigned(), input_path, wrapper }
            },)*]
        }
    }
}

make_solutions!(y2015
    day01, day02, day03, day04, day05, day06, day07
);

pub fn filtered_solutions(year: Option<u32>, day: Option<u32>) -> Vec<Solution> {
    empty()
        .chain(y2015())
        .filter(|s| year.is_none_or(|y| y == s.year))
        .filter(|s| day.is_none_or(|y| y == s.day))
        .collect()
}

#[derive(Debug)]
pub struct ParseError(String);

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse puzzle input: '{}'", self.0)
    }
}
