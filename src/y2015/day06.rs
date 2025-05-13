use std::error::Error;

use crate::util::array::CastExt;
use crate::util::parse::ParseOps;

pub enum Action {
    On,
    Off,
    Toggle,
}

pub type Coord = (usize, usize);

pub type ParsedInput = Vec<(Action, Coord, Coord)>;

pub fn parse(input: &str) -> Result<ParsedInput, Box<dyn Error>> {
    Ok(input
        .lines()
        .map(|l| {
            let action = if l.starts_with("turn on") {
                Action::On
            } else if l.starts_with("turn off") {
                Action::Off
            } else if l.starts_with("toggle") {
                Action::Toggle
            } else {
                unreachable!("valid actions are 'turn on', 'turn off', or 'toggle'")
            };

            let [x0, y0, x1, y1] = l.iter_unsigned().cast().expect("2 coords means 4 integers");

            (action, (x0, y0), (x1, y1))
        })
        .collect::<ParsedInput>())
}

pub fn part1(input: &ParsedInput) -> Option<usize> {
    let mut lights = vec![vec![false; 1000]; 1000];

    for (action, (x0, y0), (x1, y1)) in input {
        for row in &mut lights[*y0..=*y1] {
            for l in &mut row[*x0..=*x1] {
                match action {
                    Action::On => *l = true,
                    Action::Off => *l = false,
                    Action::Toggle => *l = !*l,
                };
            }
        }
    }

    lights
        .into_iter()
        .map(|row| row.iter().filter(|&&l| l).count())
        .sum::<usize>()
        .into()
}

pub fn part2(input: &ParsedInput) -> Option<u64> {
    let mut lights = vec![vec![0u64; 1000]; 1000];

    for (action, (x0, y0), (x1, y1)) in input {
        for row in &mut lights[*y0..=*y1] {
            for l in &mut row[*x0..=*x1] {
                match action {
                    Action::On => *l += 1,
                    Action::Off => *l = if *l > 0 { *l - 1 } else { 0 },
                    Action::Toggle => *l += 2,
                };
            }
        }
    }

    lights
        .into_iter()
        .map(|row| row.iter().sum::<u64>())
        .sum::<u64>()
        .into()
}
