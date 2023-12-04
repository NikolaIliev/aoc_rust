use std::{time::Instant, usize};

use aoc_rust::read_input;
use itertools::Itertools;

fn get_score(card: &str) -> usize {
    let (_, nums) = card.split_once(": ").unwrap();

    let (winning_str, own_str) = nums.split_once(" | ").unwrap();

    let winning_nums = winning_str
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();

    own_str
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .filter(|n| winning_nums.contains(n))
        .count()
}

fn part_1(input: &str) -> String {
    return input
        .lines()
        .map(|line| match get_score(line) {
            0 => 0,
            score => usize::pow(2, score as u32 - 1),
        })
        .sum::<usize>()
        .to_string();
}

fn part_2(input: &str) -> String {
    input
        .lines()
        .map(get_score)
        .enumerate()
        .fold(
            input.lines().map(|_| 1).collect_vec(),
            |mut card_counts, (i, score)| {
                for j in 0..score {
                    card_counts[i + j + 1] += card_counts[i];
                }

                card_counts
            },
        )
        .iter()
        .sum::<usize>()
        .to_string()
}

fn main() {
    let input = read_input("2023", "04");

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
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "
        .trim();

        assert_eq!(part_1(input), "13");
    }

    #[test]
    fn test_part_2() {
        let input = r"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "
        .trim();

        assert_eq!(part_2(input), "30");
    }
}
