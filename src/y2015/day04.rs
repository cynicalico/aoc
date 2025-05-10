use std::error::Error;

use md5::{Digest, Md5};

pub type ParsedInput<'a> = &'a str;

pub fn parse(input: &str) -> Result<ParsedInput, Box<dyn Error>> { Ok(input.trim()) }

pub fn part1(input: &ParsedInput) -> Option<u32> {
    for ans in 1.. {
        let hash = Md5::new_with_prefix(input).chain_update(ans.to_string()).finalize();
        if hash[0] == 0 && hash[1] == 0 && (hash[2] >> 4) & 0xf == 0 {
            return Some(ans);
        }
    }
    unreachable!();
}

pub fn part2(input: &ParsedInput) -> Option<u32> {
    for ans in 1.. {
        let hash = Md5::new_with_prefix(input).chain_update(ans.to_string()).finalize();
        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            return Some(ans);
        }
    }
    unreachable!();
}
