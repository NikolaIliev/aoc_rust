use std::time::Instant;

use aoc_rust::read_input;
use itertools::Itertools;

fn is_safe(report: &Vec<usize>, skip_index: Option<usize>) -> bool {
    let mut is_increasing: Option<bool> = None;

    for (ai, bi) in (0..report.len())
        .filter(|&i| Some(i) != skip_index)
        .tuple_windows()
    {
        let a = report[ai];
        let b = report[bi];

        match is_increasing {
            Some(true) => {
                if b <= a {
                    return false;
                }
            }
            Some(false) => {
                if b >= a {
                    return false;
                }
            }
            None => {
                if a == b {
                    return false;
                }

                is_increasing = Some(b > a)
            }
        }

        if b.abs_diff(a) > 3 {
            return false;
        }
    }

    return true;
}

fn part_1(input: &str) -> String {
    return input
        .lines()
        .map(|line| {
            let xs = line
                .split(' ')
                .map(|ch| ch.parse::<usize>().unwrap())
                .collect_vec();

            if is_safe(&xs, None) {
                1
            } else {
                0
            }
        })
        .sum::<usize>()
        .to_string();
}

fn part_2(input: &str) -> String {
    return input
        .lines()
        .map(|line| {
            let xs = line
                .split(' ')
                .map(|ch| ch.parse::<usize>().unwrap())
                .collect_vec();

            let is_any_safe = std::iter::once(None)
                .chain((0..xs.len()).map(|i| Some(i)))
                .any(|skip_index| is_safe(&xs, skip_index));

            if is_any_safe {
                1
            } else {
                0
            }
        })
        .sum::<usize>()
        .to_string();
}

fn main() {
    let input = read_input("2024", "02");

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
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
            "
        .trim();

        assert_eq!(part_1(input), "2");
    }

    #[test]
    fn test_part_2() {
        let input = r"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
            "
        .trim();

        assert_eq!(part_2(input), "4");
    }
}
