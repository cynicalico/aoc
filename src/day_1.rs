use aoc2024::read_lines;
use itertools::Itertools;
use regex::Regex;

fn main() {
    let (mut l1, mut l2) = parse_puzzle_input();

    l1.sort();
    l2.sort();

    println!("P1: {}", calculate_p1_ans(&l1, &l2));
    println!("P2: {}", calculate_p2_ans(&l1, &l2));
}

fn calculate_p1_ans(l1: &[i32], l2: &[i32]) -> i32 {
    l1.iter().zip(l2).map(|(n, m)| (n - m).abs()).sum()
}

fn calculate_p2_ans(l1: &[i32], l2: &[i32]) -> i32 {
    let l2_counts = l2.iter().counts();
    l1.iter()
        .map(|n| n * *l2_counts.get(n).unwrap_or(&0) as i32)
        .sum()
}

fn parse_puzzle_input() -> (Vec<i32>, Vec<i32>) {
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();

    read_lines("input/day_1.txt")
        .unwrap()
        .flatten()
        .map(|line| {
            let (_, [n, m]) = re.captures(line.as_str()).unwrap().extract();
            (
                n.parse::<i32>().ok().unwrap(),
                m.parse::<i32>().ok().unwrap(),
            )
        })
        .unzip()
}
