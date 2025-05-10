use std::collections::HashSet;
use std::error::Error;

#[derive(Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub type ParsedInput = Vec<Direction>;

pub fn parse(input: &str) -> Result<ParsedInput, Box<dyn Error>> {
    Ok(input
        .chars()
        .map(|c| match c {
            '^' => Direction::North,
            'v' => Direction::South,
            '>' => Direction::East,
            '<' => Direction::West,
            _ => unreachable!("Input contains only ^v<>"),
        })
        .collect())
}

fn move_in_dir(pos: (i32, i32), dir: Direction) -> (i32, i32) {
    match dir {
        Direction::North => (pos.0, pos.1 - 1),
        Direction::South => (pos.0, pos.1 + 1),
        Direction::East => (pos.0 + 1, pos.1),
        Direction::West => (pos.0 - 1, pos.1),
    }
}

pub fn part1(input: &ParsedInput) -> Option<usize> {
    let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);

    let mut santa_pos = (0, 0);

    for dir in input {
        santa_pos = move_in_dir(santa_pos, *dir);
        visited.insert(santa_pos);
    }

    visited.len().into()
}

pub fn part2(input: &ParsedInput) -> Option<usize> {
    let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);

    let mut santa_pos = (0, 0);
    let mut robo_santa_pos = (0, 0);

    for (i, dir) in input.iter().enumerate() {
        if i % 2 == 0 {
            santa_pos = move_in_dir(santa_pos, *dir);
            visited.insert(santa_pos);
        } else {
            robo_santa_pos = move_in_dir(robo_santa_pos, *dir);
            visited.insert(robo_santa_pos);
        }
    }

    visited.len().into()
}
