use std::collections::HashMap;
use std::error::Error;

use itertools::izip;

pub type ParsedInput<'a> = Vec<&'a str>;

pub fn parse(input: &str) -> Result<ParsedInput, Box<dyn Error>> { Ok(input.lines().collect()) }

fn is_string_nice_p1(s: &str) -> bool {
    let mut vowel_count = 0;
    let mut double_letter_count = 0;

    for (c1, c2) in izip!(s.chars(), s.chars().skip(1).chain(['\0'])) {
        if matches!(c1, 'a' | 'e' | 'i' | 'o' | 'u') {
            vowel_count += 1;
        }
        if c1 == c2 {
            double_letter_count += 1;
        }
        if (c1 == 'a' && c2 == 'b')
            || (c1 == 'c' && c2 == 'd')
            || (c1 == 'p' && c2 == 'q')
            || (c1 == 'x' && c2 == 'y')
        {
            return false;
        }
    }

    vowel_count >= 3 && double_letter_count >= 1
}
pub fn part1(input: &ParsedInput) -> Option<usize> {
    input.iter().filter(|s| is_string_nice_p1(s)).count().into()
}

fn is_string_nice_p2(s: &str) -> bool {
    let mut pairs: HashMap<(char, char), Vec<usize>> = HashMap::new();
    let mut double_separated_count = 0;

    for (i, (c1, c2, c3)) in izip!(
        s.chars(),
        s.chars().skip(1).chain(['\0']),
        s.chars().skip(2).chain(['\0', '\0'])
    )
    .enumerate()
    {
        pairs.entry((c1, c2)).or_default().push(i);
        if c1 == c3 {
            double_separated_count += 1;
        }
    }

    double_separated_count >= 1
        && pairs.iter().any(|(_, starts)| {
            for s1 in starts {
                for s2 in starts {
                    if s1.abs_diff(*s2) > 1 {
                        return true;
                    }
                }
            }
            false
        })
}

pub fn part2(input: &ParsedInput) -> Option<usize> {
    input.iter().filter(|s| is_string_nice_p2(s)).count().into()
}
