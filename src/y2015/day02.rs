use std::cmp;
use std::error::Error;

use crate::util::array::CastExt;
use crate::util::parse::ParseOps;

type ParsedInput = Vec<[u32; 3]>;

pub fn parse(input: &str) -> Result<ParsedInput, Box<dyn Error>> {
    Ok(input
        .lines()
        .map(|line| line.iter_unsigned::<u32>().cast().expect("lines are formatted LxWxH"))
        .collect())
}

pub fn part1(input: &ParsedInput) -> Option<u32> {
    Some(input.iter().fold(0, |acc, [l, w, h]| {
        let surface_area = 2 * l * w + 2 * w * h + 2 * h * l;
        let min_side_area = cmp::min(cmp::min(l * w, w * h), h * l);
        acc + surface_area + min_side_area
    }))
}

pub fn part2(input: &ParsedInput) -> Option<u32> {
    Some(input.iter().fold(0, |acc, [l, w, h]| {
        let min_perimeter = cmp::min(cmp::min(2 * (l + w), 2 * (w + h)), 2 * (h + l));
        let volume = l * w * h;
        acc + min_perimeter + volume
    }))
}
