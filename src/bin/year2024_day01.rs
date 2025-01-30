use std::{collections::HashMap, time::Instant};

use aoc_rust::read_input;
use itertools::Itertools;

fn parse_lists(input: &str) -> (Vec<usize>, Vec<usize>) {
    let left = input
        .lines()
        .map(|line| line.split_once("   ").unwrap().0.parse::<usize>().unwrap())
        .sorted()
        .collect_vec();

    let right = input
        .lines()
        .map(|line| line.split_once("   ").unwrap().1.parse::<usize>().unwrap())
        .sorted()
        .collect_vec();

    return (left, right);
}

fn part_1(input: &str) -> String {
    let (left, right) = parse_lists(input);
    let mut distance = 0;

    for i in 0..left.len() {
        distance += left[i].abs_diff(right[i]);
    }

    return distance.to_string();
}

fn part_2(input: &str) -> String {
    let (left, right) = parse_lists(input);
    let right_occurrences = right
        .iter()
        .fold(HashMap::<usize, usize>::new(), |mut acc, val| {
            acc.entry(*val).and_modify(|val| *val += 1).or_insert(1);

            acc
        });

    return left
        .iter()
        .map(|x| x * right_occurrences.get(x).unwrap_or(&0))
        .sum::<usize>()
        .to_string();
}

fn main() {
    let input = read_input("2024", "01");

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
3   4
4   3
2   5
1   3
3   9
3   3
            "
        .trim();

        assert_eq!(part_1(input), "11");
    }

    #[test]
    fn test_part_2() {
        let input = r"
3   4
4   3
2   5
1   3
3   9
3   3
            "
        .trim();

        assert_eq!(part_2(input), "31");
    }
}

