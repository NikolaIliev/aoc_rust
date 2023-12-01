use aoc_rust::read_input;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::time::Instant;

fn part_1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            format!(
                "{}{}",
                line.chars().find(|&c| c.is_numeric()).unwrap(),
                line.chars().rev().find(|&c| c.is_numeric()).unwrap()
            )
            .parse::<usize>()
            .unwrap()
        })
        .sum::<usize>()
        .to_string()
}

fn part_2(input: &str) -> String {
    let normal_digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let reversed_digits = [
        "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
    ];

    input
        .lines()
        .map(|line| {
            format!(
                "{}{}",
                line.chars()
                    .fold_while(String::new(), |mut acc, ch| {
                        if ch.is_numeric() {
                            Done(ch.to_string())
                        } else {
                            acc.push(ch);

                            if let Some(idx) =
                                normal_digits.iter().position(|&digit| acc.ends_with(digit))
                            {
                                let a = (b'0' + (idx as u8) + 1) as char;

                                return Done(a.to_string());
                            } else {
                                return Continue(acc);
                            }
                        }
                    })
                    .into_inner(),
                line.chars()
                    .rev()
                    .fold_while(String::new(), |mut acc, ch| {
                        if ch.is_numeric() {
                            Done(ch.to_string())
                        } else {
                            acc.push(ch);

                            if let Some(idx) = reversed_digits
                                .iter()
                                .position(|&digit| acc.ends_with(digit))
                            {
                                let a = (b'0' + (idx as u8) + 1) as char;

                                return Done(a.to_string());
                            } else {
                                return Continue(acc);
                            }
                        }
                    })
                    .into_inner()
            )
            .parse::<usize>()
            .unwrap()
        })
        .sum::<usize>()
        .to_string()
}

fn main() {
    let input = read_input("2023", "01");

    let start_part_1 = Instant::now();
    let part_1_result = part_1(&input);
    let part_1_time = start_part_1.elapsed();

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
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
        "
        .trim();

        assert_eq!(part_1(input), "142");
    }

    #[test]
    fn test_part_2() {
        let input = r"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
        "
        .trim();

        assert_eq!(part_2(input), "281");
    }

    #[test]
    fn test_part_2_2() {
        let input = r"
zerotwozerotwone
        "
        .trim();

        assert_eq!(part_2(input), "21");
    }
}
