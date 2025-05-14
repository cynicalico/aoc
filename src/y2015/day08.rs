use std::error::Error;

pub type ParsedInput = Vec<String>;

pub fn parse(input: &str) -> Result<ParsedInput, Box<dyn Error>> {
    Ok(input
        .lines()
        .map(|l| l[1..l.len() - 1].to_owned())
        .collect())
}

fn memory_len(s: &str) -> usize {
    let mut len = 0;

    let mut check_escape = false;
    let mut skip = 0;
    for c in s.chars() {
        if skip > 0 {
            skip -= 1;
        } else if check_escape {
            match c {
                '\\' | '"' => (),
                'x' => skip += 2,
                _ => unreachable!("Input is good on account of being a puzzle"),
            }
            len += 1;
            check_escape = false;
        } else if c == '\\' {
            check_escape = true;
        } else {
            len += 1;
        }
    }

    len
}

pub fn part1(input: &ParsedInput) -> Option<i64> {
    let total_code_len = input.iter().map(|l| l.len() + 2).sum::<usize>() as i64;
    let total_memory_len = input.iter().map(|l| memory_len(l)).sum::<usize>() as i64;

    Some(total_code_len - total_memory_len)
}

fn encoded_len(s: &str) -> usize {
    s.chars()
        .map(|c| match c {
            '\\' => 2,
            '"' => 2,
            _ => 1,
        })
        .sum::<usize>()
        + 6
}

pub fn part2(input: &ParsedInput) -> Option<i64> {
    let total_code_len = input.iter().map(|l| l.len() + 2).sum::<usize>() as i64;
    let total_encoded_len = input.iter().map(|l| encoded_len(l)).sum::<usize>() as i64;

    Some(total_encoded_len - total_code_len)
}
