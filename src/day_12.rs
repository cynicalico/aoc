/* https://adventofcode.com/2024/day/12
 */

use aoc2024::read_lines;
use itertools::Itertools;

fn main() {
    let start = std::time::Instant::now();

    let mut map = parse_puzzle_input();
    map.insert(0, vec!['\0'; map[0].len()]);
    map.push(vec!['\0'; map[0].len()]);

    println!("P1: {}", calculate_p1_ans(&map));
    println!("P1: {}", calculate_p2_ans(&map));
    println!("Took {:.04}s", start.elapsed().as_nanos() as f64 / 1e9);
}

fn calculate_p1_ans(map: &[Vec<char>]) -> i32 {
    let mut price = 0;
    let mut visited = vec![vec![false; map[0].len()]; map.len()];

    for y in 1..map.len() - 1 {
        for x in 1..map[y].len() - 1 {
            if !visited[y][x] {
                price += flood_region(map, &mut visited, y, x);
            }
        }
    }

    price
}

fn calculate_p2_ans(map: &[Vec<char>]) -> i32 {
    let mut price = 0;
    let mut visited = vec![vec![false; map[0].len()]; map.len()];

    for y in 1..map.len() - 1 {
        for x in 1..map[y].len() - 1 {
            if !visited[y][x] {
                price += flood_region_bulk_discount(map, &mut visited, y, x);
            }
        }
    }

    price
}

fn flood_region(
    map: &[Vec<char>],
    visited: &mut [Vec<bool>],
    start_y: usize,
    start_x: usize,
) -> i32 {
    let mut area = 0;
    let mut perimeter = 0;

    let region_char = map[start_y][start_x];

    let mut stack = Vec::from([(start_y, start_x)]);
    while !stack.is_empty() {
        let p = stack.pop().unwrap();
        if !visited[p.0][p.1] {
            area += 1;

            for o in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let op = ((p.0 as i32 + o.0) as usize, (p.1 as i32 + o.1) as usize);
                if map[op.0][op.1] != region_char {
                    perimeter += 1;
                } else {
                    stack.push(op);
                }
            }
        }
        visited[p.0][p.1] = true;
    }

    area * perimeter
}

fn flood_region_bulk_discount(
    map: &[Vec<char>],
    visited: &mut [Vec<bool>],
    start_y: usize,
    start_x: usize,
) -> i32 {
    let mut area = 0;
    let mut corners = 0;

    let region_char = map[start_y][start_x];

    let mut stack = Vec::from([(start_y, start_x)]);
    while !stack.is_empty() {
        let p = stack.pop().unwrap();
        if !visited[p.0][p.1] {
            area += 1;

            let mut kernel = [['\0'; 3]; 3];
            kernel[1][1] = map[p.0][p.1];
            for o in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let op = ((p.0 as i32 + o.0) as usize, (p.1 as i32 + o.1) as usize);
                kernel[(o.0 + 1) as usize][(o.1 + 1) as usize] = map[op.0][op.1];
                if map[op.0][op.1] == region_char {
                    stack.push(op);
                }
            }
            for o in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
                let op = ((p.0 as i32 + o.0) as usize, (p.1 as i32 + o.1) as usize);
                kernel[(o.0 + 1) as usize][(o.1 + 1) as usize] = map[op.0][op.1];
            }

            for ((p00, p01), (p10, p11)) in [
                ((0, 1), (1, 0)),
                ((0, 1), (1, 2)),
                ((2, 1), (1, 0)),
                ((2, 1), (1, 2)),
            ] {
                // outer corner
                if kernel[p00][p01] != kernel[1][1] && kernel[p10][p11] != kernel[1][1] {
                    corners += 1;
                }

                // inside corner
                if kernel[p00][p01] == kernel[1][1]
                    && kernel[p10][p11] == kernel[1][1]
                    && kernel[p00][p11] != kernel[1][1]
                {
                    corners += 1;
                }
            }
        }
        visited[p.0][p.1] = true;
    }

    area * corners
}

fn parse_puzzle_input() -> Vec<Vec<char>> {
    read_lines("input/day_12.txt")
        .expect("Failed to open input file")
        .flatten()
        .map(|line| format!("\0{line}\0").chars().collect_vec())
        .collect_vec()
}