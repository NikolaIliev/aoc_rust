use aoc_rust::read_input;
use itertools::Itertools;

fn priority(c: char) -> usize {
    if c <= 'Z' {
        c as usize - 'A' as usize + 27
    } else {
        c as usize - 'a' as usize + 1
    }
}

fn part_1(input: &str) -> String {
    return input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(c1, c2)| {
            c1.chars().find_map(|c| {
                if c2.chars().contains(&c) {
                    Some(priority(c))
                } else {
                    None
                }
            })
        })
        .map(|p| p.unwrap())
        .sum::<usize>()
        .to_string();
}

fn part_2(input: &str) -> String {
    return input
        .lines()
        .tuples()
        .map(|(a, b, c)| {
            priority(
                a.chars()
                    .find(|x| b.chars().contains(x) && c.chars().contains(x))
                    .unwrap(),
            )
        })
        .sum::<usize>()
        .to_string();
}

fn main() {
    let input = read_input("2022", "03");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
        "
        .trim();

        assert_eq!(part_1(input), "157");
    }

    #[test]
    fn test_part_2() {
        let input = r"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
        "
        .trim();

        assert_eq!(part_2(input), "70");
    }
}
