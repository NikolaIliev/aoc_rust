use std::time::Instant;

use aoc_rust::read_input;
use itertools::Itertools;

fn predict_next(row: Vec<isize>) -> isize {
    let mut rows: Vec<Vec<isize>> = vec![row];
    let mut all_zeroes = false;

    while !all_zeroes {
        all_zeroes = true;
        let last_row = rows.last().unwrap();
        let mut new_row: Vec<isize> = vec![];

        for i in 0..last_row.len() - 1 {
            let a = last_row[i];
            let b = last_row[i + 1];
            let n = b - a;

            new_row.push(n);

            if n != 0 {
                all_zeroes = false;
            }
        }

        rows.push(new_row);
    }

    let mut to_add: isize = 0;

    for i in (0..rows.len() - 1).rev() {
        let new_el = rows[i].last().unwrap() + to_add;

        rows[i].push(new_el);

        to_add = new_el;
    }

    rows[0].last().unwrap().clone()
}

fn part_1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            predict_next(
                line.split_whitespace()
                    .map(|s| s.parse::<isize>().unwrap())
                    .collect_vec(),
            )
        })
        .sum::<isize>()
        .to_string()
}

fn part_2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            predict_next(
                line.split_whitespace()
                    .map(|s| s.parse::<isize>().unwrap())
                    .rev()
                    .collect_vec(),
            )
        })
        .sum::<isize>()
        .to_string()
}

fn main() {
    let input = read_input("2023", "09");

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
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
        "
        .trim();

        assert_eq!(part_1(input), "114");
    }

    #[test]
    fn test_part_2() {
        let input = r"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
        "
        .trim();

        assert_eq!(part_2(input), "2");
    }
}
