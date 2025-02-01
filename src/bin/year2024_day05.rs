use std::{collections::HashMap, time::Instant};

use aoc_rust::read_input;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, usize},
    combinator::opt,
    multi::{fold_many0, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};

fn parse_rule(input: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(usize, tag("|"), usize).parse(input)
}

fn parse_forbidden_suffixes(input: &str) -> IResult<&str, HashMap<usize, Vec<usize>>> {
    fold_many0(
        (parse_rule, opt(newline)),
        HashMap::<usize, Vec<usize>>::new,
        |mut forbidden_suffixes, ((left, right), _)| {
            forbidden_suffixes
                .entry(right)
                .and_modify(|vec| vec.push(left))
                .or_insert(vec![left]);

            forbidden_suffixes
        },
    )
    .parse(input)
}

fn parse_updates(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list1(newline, separated_list1(tag(","), usize)).parse(input)
}

fn part_1(input: &str) -> String {
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();
    let forbidden_suffixes = parse_forbidden_suffixes(rules_str).unwrap().1;
    let updates = parse_updates(updates_str).unwrap().1;

    updates
        .iter()
        .map(|update| {
            let mut forbidden_ns = Vec::<usize>::new();

            for n in update {
                if forbidden_ns.iter().any(|x| x == n) {
                    return 0;
                }

                if let Some(fs) = forbidden_suffixes.get(n) {
                    forbidden_ns.extend(fs);
                }
            }

            return update[update.len() / 2];
        })
        .sum::<usize>()
        .to_string()
}

fn part_2(input: &str) -> String {
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();
    let forbidden_suffixes = parse_forbidden_suffixes(rules_str).unwrap().1;
    let updates = parse_updates(updates_str).unwrap().1;

    updates
        .iter()
        .map(|update| {
            let mut fixed_update: Vec<usize> = vec![];
            let mut forbidden_ns = Vec::<usize>::new();
            let mut did_fix_update = false;

            for &n in update {
                if forbidden_ns.iter().any(|&x| x == n) {
                    did_fix_update = true;

                    // if forbidden, look for the first entry in the fixed_update
                    // which forbids n as a suffix
                    // then insert n at that index
                    for i in 0..fixed_update.len() {
                        if let Some(fs) = forbidden_suffixes.get(&fixed_update[i]) {
                            if fs.contains(&n) {
                                fixed_update.insert(i, n);
                                break;
                            }
                        }
                    }
                } else {
                    // n is not a forbidden suffix for any preceding number -> just Pushit
                    fixed_update.push(n);
                }

                if let Some(fs) = forbidden_suffixes.get(&n) {
                    forbidden_ns.extend(fs);
                }
            }

            if did_fix_update {
                fixed_update[update.len() / 2]
            } else {
                0
            }
        })
        .sum::<usize>()
        .to_string()
}

fn main() {
    let input = read_input("2024", "05");

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
    fn test_parse_rule() {
        assert_eq!(parse_rule("420|69"), Ok(("", ((420, 69)))));
    }

    #[test]
    fn test_parse_updates() {
        assert_eq!(
            parse_updates("1,2,3\n69,420"),
            Ok(("", vec![vec![1, 2, 3], vec![69, 420]]))
        );
    }

    #[test]
    fn test_part_1() {
        let input = r"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
            "
        .trim();

        assert_eq!(part_1(input), "143");
    }

    #[test]
    fn test_part_2() {
        let input = r"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
            "
        .trim();

        assert_eq!(part_2(input), "123");
    }
}

