use std::time::Instant;

use aoc_rust::read_input;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until},
    character::complete::{anychar, usize},
    combinator::{cut, opt},
    multi::{fold_many0, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

fn parse_num_pair(input: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(usize, tag(","), usize).parse(input)
}

fn parse_expression(input: &str) -> IResult<&str, (usize, usize)> {
    delimited(tag("("), parse_num_pair, tag(")")).parse(input)
}

fn part_1(input: &str) -> String {
    let res = fold_many0(
        (take_until("mul"), cut(tag("mul")), opt(parse_expression)),
        || 0 as usize,
        |acc, (_, _, pair)| match pair {
            Some((a, b)) => acc + a * b,
            None => acc,
        },
    )
    .parse(input)
    .unwrap()
    .1;

    return res.to_string();
}

struct ParseState {
    skipping: bool,
    sum: usize,
}

impl ParseState {
    fn new() -> ParseState {
        return ParseState {
            skipping: false,
            sum: 0,
        };
    }
}

fn part_2(input: &str) -> String {
    let res = fold_many0(
        (
            many_till(
                anychar::<&str, nom::error::Error<&str>>,
                alt((tag("mul"), tag("do()"), tag("don't()"))),
            ),
            opt(parse_expression),
        ),
        ParseState::new,
        |mut state, ((_, keyword), m)| {
            match keyword {
                "do()" => state.skipping = false,
                "don't()" => state.skipping = true,
                "mul" if !state.skipping => {
                    if let Some((a, b)) = m {
                        state.sum += a * b;
                    }
                }
                _ => (),
            }

            state
        },
    )
    .parse(input)
    .unwrap()
    .1
    .sum;

    return res.to_string();
}

fn main() {
    let input = read_input("2024", "03");

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
    fn test_num_pair() {
        assert_eq!(parse_num_pair("23,45"), Ok(("", (23, 45))));
    }

    #[test]
    fn test_expression() {
        assert_eq!(parse_expression("(23,45)"), Ok(("", (23, 45))));
        assert_eq!(parse_expression("(999,1)"), Ok(("", (999, 1))));
    }

    #[test]
    fn test_part_1() {
        let input = r"
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
            "
        .trim();

        assert_eq!(part_1(input), "161");
    }

    #[test]
    fn test_part_2() {
        let input = r"
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
            "
        .trim();

        assert_eq!(part_2(input), "48");
    }
}

