use crate::util::{io::read_lines_partitioned, trie::*};
use hashbrown::HashMap;
use std::io;

type Input = (Trie, Vec<String>);

pub fn parse(filepath: &str) -> io::Result<Input> {
    let mut trie = Trie::new();
    let mut designs = Vec::new();

    read_lines_partitioned(
        filepath,
        |line| {
            line.split(", ").for_each(|pattern| trie.insert(pattern));
        },
        |line| {
            designs.push(line);
        },
    )?;

    Ok((trie, designs))
}

pub fn part1(input: &Input) -> Option<usize> {
    let (trie, designs) = input;
    let mut memo: HashMap<String, bool> = HashMap::new();
    designs.iter().filter(|d| is_possible(&trie, d, &mut memo)).count().into()
}

pub fn part2(input: &Input) -> Option<u64> {
    let (trie, designs) = input;
    let mut memo: HashMap<String, u64> = HashMap::new();
    designs.iter().map(|d| count_possible(&trie, d, &mut memo)).sum::<u64>().into()
}

fn is_possible(trie: &Trie, d: &str, memo: &mut HashMap<String, bool>) -> bool {
    *match memo.get(d) {
        Some(b) => b,
        None => {
            let res = trie.find(d)
                || (1..d.len().min(trie.max_key_len + 1))
                    .rev()
                    .any(|i| trie.find(&d[..i]) && is_possible(trie, &d[i..], memo));
            memo.entry_ref(d).or_insert(res)
        }
    }
}

fn count_possible(trie: &Trie, d: &str, memo: &mut HashMap<String, u64>) -> u64 {
    *match memo.get(d) {
        Some(c) => c,
        None => {
            let mut total = if trie.find(d) { 1 } else { 0 };
            for i in (1..d.len().min(trie.max_key_len + 1)).rev() {
                if trie.find(&d[..i]) {
                    total += count_possible(trie, &d[i..], memo);
                }
            }
            memo.entry_ref(d).or_insert(total)
        }
    }
}
