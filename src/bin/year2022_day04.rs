use aoc_rust::read_input;
use itertools::Itertools;

fn parse_range(s: &str) -> (usize, usize) {
    let mut parts = s.split('-');

    (
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
    )
}

fn part_1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .tuples()
                .map(|(a, b)| (parse_range(a), parse_range(b)))
                .map(|((a, b), (c, d))| {
                    if a >= c && b <= d || c >= a && d <= b {
                        1
                    } else {
                        0
                    }
                })
        })
        .flatten()
        .sum::<usize>()
        .to_string()
}

fn part_2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .tuples()
                .map(|(a, b)| (parse_range(a), parse_range(b)))
                .map(|((a, b), (c, d))| if a <= d && b >= c { 1 } else { 0 })
        })
        .flatten()
        .sum::<usize>()
        .to_string()
}

fn main() {
    let input = read_input("2022", "04");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
        "
        .trim();

        assert_eq!(part_1(input), "2");
    }

    #[test]
    fn test_part_2() {
        let input = r"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
        "
        .trim();

        assert_eq!(part_2(input), "4");
    }
}
