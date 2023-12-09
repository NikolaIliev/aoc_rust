use std::time::Instant;

use aoc_rust::read_input;
use itertools::Itertools;

fn part_1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let dims = line
                .split("x")
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec();
            let a = dims[0];
            let b = dims[1];
            let c = dims[2];
            let area1 = a * b;
            let area2 = a * c;
            let area3 = b * c;
            let min_area = area1.min(area2).min(area3);

            2 * area1 + 2 * area2 + 2 * area3 + min_area
        })
        .sum::<usize>()
        .to_string()
}

fn part_2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let dims = line
                .split("x")
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec();
            let a = dims[0];
            let b = dims[1];
            let c = dims[2];
            let smallest = a.min(b).min(c);
            let largest = a.max(b).max(c);
            let middle = a + b + c - smallest - largest;
            let volume = a * b * c;

            volume + 2 * smallest + 2 * middle
        })
        .sum::<usize>()
        .to_string()
}

fn main() {
    let input = read_input("2015", "02");

    let start_part_1 = Instant::now();
    let part_1_result = part_1(&input);
    let part_1_time = start_part_1.elapsed();

    println!();
    println!("Part 1: {} ({:?})", part_1_result, part_1_time);

    let start_part_2 = Instant::now();
    let part_2_result = part_2(&input);
    let part_2_time = start_part_2.elapsed();

    println!("Part 2: {} ({:?})", part_2_result, part_2_time);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r"
2x3x4
1x1x10"
            .trim();

        assert_eq!(part_1(input), "101");
    }

    #[test]
    fn test_part_2() {
        let input = r"

3x4x2
10x1x1"
            .trim();

        assert_eq!(part_2(input), "48");
    }
}
