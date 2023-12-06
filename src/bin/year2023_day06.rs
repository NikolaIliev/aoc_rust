use itertools::Itertools;
use std::time::Instant;

use aoc_rust::read_input;

fn solve_quadratic(a: f64, b: f64, c: f64) -> (f64, f64) {
    return (
        (-b + (b * b - 4.0 * a * c).sqrt()) / 2.0 * a,
        (-b - (b * b - 4.0 * a * c).sqrt()) / 2.0 * a,
    );
}

fn get_solutions_count(time: usize, distance: usize) -> usize {
    let a = -1 as f64;
    let b = time as f64;
    // add some small n so that we actually beat the record and not match it
    let c = -(distance as f64 + 0.01);

    let (x1, x2) = solve_quadratic(a, b, c);

    (x2.floor() - x1.ceil() + 1.0).round() as usize
}

fn part_1(input: &str) -> String {
    let (time_str, distance_str) = input.split_once("\n").unwrap();

    time_str
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .zip(
            distance_str
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok()),
        )
        .map(|(time, distance)| get_solutions_count(time, distance))
        .product::<usize>()
        .to_string()
}

fn part_2(input: &str) -> String {
    let (time_str, distance_str) = input.split_once("\n").unwrap();

    get_solutions_count(
        time_str
            .split_whitespace()
            .filter(|s| s.parse::<usize>().is_ok())
            .join("")
            .parse::<usize>()
            .unwrap(),
        distance_str
            .split_whitespace()
            .filter(|s| s.parse::<usize>().is_ok())
            .join("")
            .parse::<usize>()
            .unwrap(),
    )
    .to_string()
}

fn main() {
    let input = read_input("2023", "06");

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
Time:      7  15   30
Distance:  9  40  200
        "
        .trim();

        assert_eq!(part_1(input), "288");
    }

    #[test]
    fn test_part_2() {
        let input = r"
Time:      7  15   30
Distance:  9  40  200
        "
        .trim();

        assert_eq!(part_2(input), "71503");
    }
}
