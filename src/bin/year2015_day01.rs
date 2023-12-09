use std::time::Instant;

use aoc_rust::read_input;
use itertools::{FoldWhile, Itertools};

fn part_1(input: &str) -> String {
    input
        .chars()
        .fold(0, |floor, ch| match ch {
            '(' => floor + 1,
            ')' => floor - 1,
            _ => floor,
        })
        .to_string()
}

fn part_2(input: &str) -> String {
    input
        .chars()
        .enumerate()
        .fold_while(0, |floor, (idx, ch)| {
            if floor == -1 {
                FoldWhile::Done(idx as isize)
            } else {
                match ch {
                    '(' => FoldWhile::Continue(floor + 1),
                    ')' => FoldWhile::Continue(floor - 1),
                    _ => panic!(),
                }
            }
        })
        .into_inner()
        .to_string()
}

fn main() {
    let input = read_input("2015", "01");

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

    #[ignore]
    #[test]
    fn test_part_1() {
        let input = r"))(((((".trim();

        assert_eq!(part_1(input), "3");
    }

    #[ignore]
    #[test]
    fn test_part_2() {
        let input = r"".trim();

        assert_eq!(part_2(input), "");
    }
}
