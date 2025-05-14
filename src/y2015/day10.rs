use std::error::Error;
use std::iter::once;

use itertools::Itertools;

pub type ParsedInput = (usize, usize);

fn say(seq: &str) -> String {
    let mut s = String::new();
    s.reserve((seq.len() as f64 * 1.3) as usize);

    let mut n = 0;
    for (c1, c2) in once(seq.chars().next().unwrap())
        .chain(seq.chars())
        .chain(once('\0'))
        .tuple_windows()
    {
        if c2 != c1 {
            s += &format!("{n}{c1}");
            n = 0;
        }
        n += 1;
    }

    s
}

pub fn parse(input: &str) -> Result<ParsedInput, Box<dyn Error>> {
    let mut seq = input.trim().to_string();

    for _ in 0..40 {
        seq = say(&seq);
    }
    let p1_ans = seq.len();

    for _ in 0..10 {
        seq = say(&seq);
    }

    Ok((p1_ans, seq.len()))
}

pub fn part1(input: &ParsedInput) -> Option<usize> { input.0.into() }

pub fn part2(input: &ParsedInput) -> Option<usize> { input.1.into() }
