use aoc_rust::read_input;
use itertools::Itertools;

fn part_1(input: &str) -> String {
    return input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
        .to_string();
}

fn part_2(input: &str) -> String {
    return input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .sum::<u32>()
        .to_string();
}

fn main() {
    let input = read_input("2022", "01");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
            .trim();

        assert_eq!(part_1(input), "24000");
    }

    #[test]
    fn test_part_2() {
        let input = r"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
            .trim();

        assert_eq!(part_2(input), "45000");
    }
}
