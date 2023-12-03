use std::{collections::HashSet, time::Instant};

use aoc_rust::read_input;
use itertools::Itertools;

fn is_engine_part(ch: char) -> bool {
    return ch != '.' && !ch.is_numeric();
}

fn is_gear(ch: char) -> bool {
    return ch == '*';
}

fn get_coords_of_adjacent_chars_that_match_predicate(
    matrix: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    predicate: fn(char) -> bool,
) -> Vec<(usize, usize)> {
    let x_i = x as isize;
    let y_i = y as isize;

    vec![
        (x_i - 1, y_i - 1),
        (x_i, y_i - 1),
        (x_i + 1, y_i - 1),
        (x_i - 1, y_i),
        (x_i + 1, y_i),
        (x_i - 1, y_i + 1),
        (x_i, y_i + 1),
        (x_i + 1, y_i + 1),
    ]
    .iter()
    .filter(|(x, y)| {
        *x >= 0
            && *y >= 0
            && *x < matrix[0].len() as isize
            && *y < matrix.len() as isize
            && predicate(matrix[*y as usize][*x as usize])
    })
    .map(|(x, y)| (*x as usize, *y as usize))
    .collect_vec()
}

fn part_1(input: &str) -> String {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut sum: usize = 0;

    for y in 0..matrix.len() {
        let line = &matrix[y];
        let mut num_str = String::new();
        let mut num_has_adjacent_engine_part = false;

        for x in 0..=line.len() {
            let ch = if x < line.len() { line[x] } else { '.' };

            if ch.is_numeric() {
                num_str.push(ch);
                if !num_has_adjacent_engine_part {
                    num_has_adjacent_engine_part =
                        !get_coords_of_adjacent_chars_that_match_predicate(
                            &matrix,
                            x,
                            y,
                            is_engine_part,
                        )
                        .is_empty();
                }
            } else {
                if !num_str.is_empty() {
                    if num_has_adjacent_engine_part {
                        let num = num_str.parse::<usize>().unwrap();
                        sum += num;
                    }

                    num_has_adjacent_engine_part = false;
                    num_str.clear()
                }
            }
        }
    }

    return sum.to_string();
}

fn part_2(input: &str) -> String {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut gear_counts = input
        .lines()
        .map(|line| line.chars().map(|_| 0).collect_vec())
        .collect_vec();

    let mut gear_ratios = input
        .lines()
        .map(|line| line.chars().map(|_| 1).collect_vec())
        .collect_vec();

    let mut sum = 0;

    for y in 0..matrix.len() {
        let line = &matrix[y];
        let mut num_str = String::new();
        let mut adjacent_gears: HashSet<(usize, usize)> = HashSet::new();

        for x in 0..=line.len() {
            let ch = if x < line.len() { line[x] } else { '.' };

            if ch.is_numeric() {
                num_str.push(ch);

                for (x, y) in
                    get_coords_of_adjacent_chars_that_match_predicate(&matrix, x, y, is_gear)
                {
                    adjacent_gears.insert((x, y));
                }
            } else if !num_str.is_empty() {
                let num = num_str.parse::<usize>().unwrap();

                for (x, y) in adjacent_gears.iter() {
                    gear_counts[*y][*x] += 1;

                    if gear_counts[*y][*x] <= 2 {
                        gear_ratios[*y][*x] *= num;
                    }
                }

                num_str.clear();
                adjacent_gears.clear();
            }
        }
    }

    for y in 0..gear_counts.len() {
        for x in 0..gear_counts[y].len() {
            if gear_counts[y][x] == 2 {
                sum += gear_ratios[y][x];
            }
        }
    }

    return sum.to_string();
}

fn main() {
    let input = read_input("2023", "03");

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
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .trim();

        assert_eq!(part_1(input), "4361");
    }

    #[test]
    fn test_part_2() {
        let input = r"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .trim();

        assert_eq!(part_2(input), "467835");
    }
}
