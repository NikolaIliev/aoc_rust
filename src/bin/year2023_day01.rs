use aoc_rust::read_input;

fn part_1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut iter = line.chars().filter(|c| c.is_numeric());
            let first = iter.next().unwrap();

            return format!("{}{}", first, iter.rev().next().unwrap_or(first))
                .parse::<usize>()
                .unwrap();
        })
        .sum::<usize>()
        .to_string()
}

fn part_2(input: &str) -> String {
    let digits = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    input
        .lines()
        .map(|line| {
            let mut iter = line
                .chars()
                .scan(String::new(), |state, c| {
                    if c.is_numeric() {
                        *state = String::new();

                        return Some(c);
                    }

                    state.push(c);

                    if let Some(idx) = digits.iter().position(|&digit| state.ends_with(digit)) {
                        return Some((b'0' + (idx as u8) + 1) as char);
                    }

                    return Some('_');
                })
                .filter(|c| c.is_numeric());

            let first = iter.next().unwrap();

            format!("{}{}", first, iter.last().unwrap_or(first))
                .parse::<usize>()
                .unwrap()
        })
        .sum::<usize>()
        .to_string()
}

fn main() {
    let input = read_input("2023", "01");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
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
