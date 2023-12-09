use std::{collections::HashSet, time::Instant};

use aoc_rust::read_input;

fn part_1(input: &str) -> String {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert((0, 0));

    let mut current = (0, 0);

    for ch in input.chars() {
        match ch {
            '>' => current.0 += 1,
            '<' => current.0 -= 1,
            '^' => current.1 += 1,
            'v' => current.1 -= 1,
            _ => {}
        }

        visited.insert(current);
    }

    visited.len().to_string()
}

fn part_2(input: &str) -> String {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert((0, 0));

    let mut current_a = (0, 0);
    let mut current_b = (0, 0);

    for (idx, ch) in input.chars().enumerate() {
        let current = if idx % 2 == 0 {
            &mut current_a
        } else {
            &mut current_b
        };

        match ch {
            '>' => current.0 += 1,
            '<' => current.0 -= 1,
            '^' => current.1 += 1,
            'v' => current.1 -= 1,
            _ => {}
        }

        visited.insert(*current);
    }

    visited.len().to_string()
}

fn main() {
    let input = read_input("2015", "03");

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
        let input = r"^>v<".trim();

        assert_eq!(part_1(input), "4");
    }

    #[ignore]
    #[test]
    fn test_part_2() {
        let input = r"".trim();

        assert_eq!(part_2(input), "");
    }
}
