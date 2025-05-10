use std::collections::HashMap;
use std::error::Error;

pub type ParsedInput<'a> = &'a str;

pub fn parse(input: &str) -> Result<ParsedInput, Box<dyn Error>> { Ok(input) }

pub fn part1(input: &ParsedInput) -> Option<i32> {
    let counts: HashMap<char, usize> = input.chars().fold(HashMap::default(), |mut counts, c| {
        *counts.entry(c).or_insert(0) += 1;
        counts
    });

    Some(counts[&'('] as i32 - counts[&')'] as i32)
}

pub fn part2(input: &ParsedInput) -> Option<usize> {
    let mut floor = 0;
    for (i, c) in input.char_indices() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Unexpected character in input string: '{c}'"),
        }
        if floor == -1 {
            return Some(i + 1);
        }
    }

    panic!("Santa never enters the basement");
}
