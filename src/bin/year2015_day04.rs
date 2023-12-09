use std::time::Instant;

use aoc_rust::read_input;

fn part_1(input: &str) -> String {
    let mut n = 0;

    loop {
        let s = format!("{input}{}", n);
        let hash = md5::compute(&s);
        let hash_str = format!("{:x}", hash);

        if &hash_str[0..5] == "00000" {
            return n.to_string();
        }

        n += 1;
    }
}

fn part_2(input: &str) -> String {
    let mut n = 0;

    loop {
        let s = format!("{input}{}", n);
        let hash = md5::compute(&s);
        let hash_str = format!("{:x}", hash);

        if &hash_str[0..6] == "000000" {
            return n.to_string();
        }

        n += 1;
    }
}

fn main() {
    let input = read_input("2015", "04");

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
        let input = r"abcdef".trim();

        assert_eq!(part_1(input), "609043");
    }

    #[ignore]
    #[test]
    fn test_part_2() {
        let input = r"".trim();

        assert_eq!(part_2(input), "");
    }
}
