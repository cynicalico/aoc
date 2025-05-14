use std::cmp::Ordering;
use std::error::Error;

use crate::util::array::CastExt;
use crate::util::id_cache::IdCache;
use crate::util::parse::ParseOps;

pub type ParsedInput = Vec<Vec<u64>>;

pub fn parse(input: &str) -> Result<ParsedInput, Box<dyn Error>> {
    let lines: Vec<[&str; 5]> = input
        .lines()
        .map(|l| l.split_whitespace().cast().unwrap())
        .collect();

    let mut ids = IdCache::default();
    for line in &lines {
        ids.get_or_add(line[0].to_owned());
        ids.get_or_add(line[2].to_owned());
    }

    let mut dm = vec![vec![0u64; ids.len()]; ids.len()];
    for line in &lines {
        let from = ids.get(line[0]);
        let to = ids.get(line[2]);
        let dist: u64 = line[4].unsigned();
        dm[from][to] = dist;
        dm[to][from] = dist;
    }

    Ok(dm)
}

fn opti_hamilton(ord: Ordering, dm: &ParsedInput, from: usize, visited: &mut [bool]) -> u64 {
    let sentinel = if ord == Ordering::Less {
        u64::MAX
    } else {
        u64::MIN
    };
    let mut opti_len = sentinel;

    visited[from] = true;
    for to in 0..visited.len() {
        if visited[to] {
            continue;
        }

        let d = dm[from][to] + opti_hamilton(ord, dm, to, visited);
        if d.cmp(&opti_len) == ord {
            opti_len = d;
        }
    }
    visited[from] = false;

    if opti_len == sentinel { 0 } else { opti_len }
}

pub fn part1(input: &ParsedInput) -> Option<u64> {
    let mut visited = vec![false; input.len()];
    (0..input.len())
        .map(|start| opti_hamilton(Ordering::Less, input, start, &mut visited))
        .min()
}

pub fn part2(input: &ParsedInput) -> Option<u64> {
    let mut visited = vec![false; input.len()];
    (0..input.len())
        .map(|start| opti_hamilton(Ordering::Greater, input, start, &mut visited))
        .max()
}
