use std::{collections::HashMap, time::Instant};

use aoc_rust::read_input;
use itertools::Itertools;

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Vec<usize>>) {
    input.lines().fold(
        (Vec::<Vec<char>>::new(), Vec::<Vec<usize>>::new()),
        |(mut chars, mut counts), line| {
            let (left, right) = line.split_once(" ").unwrap();

            chars.push(left.chars().collect_vec());
            counts.push(
                right
                    .split(",")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect_vec(),
            );

            (chars, counts)
        },
    )
}

fn parse_input_part2(input: &str) -> (Vec<Vec<char>>, Vec<Vec<usize>>) {
    input.lines().fold(
        (Vec::<Vec<char>>::new(), Vec::<Vec<usize>>::new()),
        |(mut chars, mut counts), line| {
            let (left, right) = line.split_once(" ").unwrap();

            let mut ch = left.chars().collect_vec();
            ch.push('?');
            ch = ch.repeat(5);
            ch.pop();

            chars.push(ch);

            counts.push(
                right
                    .split(",")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect_vec()
                    .repeat(5),
            );

            (chars, counts)
        },
    )
}

fn find_solutions(
    chars: &Vec<char>,
    counts: &Vec<usize>,
    char: Option<&char>,
    char_idx: usize,
    count_idx: usize,
    current_count_idx_found: usize,
    cache: &mut HashMap<(usize, usize, usize, char), usize>, //path: Vec<char>,
) -> usize {
    //println!(
    //"{char_idx}, {count_idx}, {current_count_idx_found} -> {:?}",
    //if let Some(ch) = char { ch } else { &'?' }
    //);

    // reached end
    if char.is_none() {
        return if count_idx == counts.len() - 1 && current_count_idx_found == counts[count_idx]
            || count_idx == counts.len() && current_count_idx_found == 0
        {
            // filled all requirements
            //println!("FOUND SOLUTION");
            1
        } else {
            //println!("NO SOLUTION");
            0
        };
    }

    let ch = char.unwrap();

    if let Some(count) = cache.get(&(char_idx, count_idx, current_count_idx_found, *ch)) {
        return *count;
    }

    //let mut new_path = path.clone();
    //
    //if char.unwrap() != &'?' {
    //new_path.push(*char.unwrap());
    //}

    let count = match ch {
        '.' => {
            if current_count_idx_found > 0 && current_count_idx_found < counts[count_idx] {
                return 0;
            }

            find_solutions(
                chars,
                counts,
                chars.get(char_idx + 1),
                char_idx + 1,
                if current_count_idx_found > 0 {
                    // finish chunk
                    count_idx + 1
                } else {
                    count_idx
                },
                0,
                cache,
            )
        }
        '#' => {
            if count_idx >= counts.len() || current_count_idx_found + 1 > counts[count_idx] {
                //println!("too many #, not a solution");
                // too many #s found, not a solution
                return 0;
            }

            find_solutions(
                chars,
                counts,
                chars.get(char_idx + 1),
                char_idx + 1,
                count_idx,
                current_count_idx_found + 1,
                cache,
            )
        }
        '?' => {
            //println!("FORKING({char_idx}) .");
            let dot = find_solutions(
                chars,
                counts,
                Some(&'.'),
                char_idx,
                count_idx,
                current_count_idx_found,
                cache,
            );

            //println!("FORKING({char_idx}) #");
            let hash = find_solutions(
                chars,
                counts,
                Some(&'#'),
                char_idx,
                count_idx,
                current_count_idx_found,
                cache,
            );

            //println!("FORK({char_idx}) RESULT: {dot} + {hash}");

            dot + hash
        }
        _ => panic!(),
    };

    cache.insert((char_idx, count_idx, current_count_idx_found, *ch), count);

    count
}

fn part_1(input: &str) -> String {
    let (chars, counts) = parse_input(input);

    chars
        .iter()
        .enumerate()
        .map(|(idx, chars)| {
            find_solutions(
                chars,
                &counts[idx],
                Some(&chars[0]),
                0,
                0,
                0,
                &mut HashMap::<(usize, usize, usize, char), usize>::new(),
            )
        })
        .sum::<usize>()
        .to_string()
}

fn part_2(input: &str) -> String {
    let (chars, counts) = parse_input_part2(input);

    chars
        .iter()
        .enumerate()
        .map(|(idx, chars)| {
            find_solutions(
                chars,
                &counts[idx],
                Some(&chars[0]),
                0,
                0,
                0,
                &mut HashMap::<(usize, usize, usize, char), usize>::new(),
            )
        })
        .sum::<usize>()
        .to_string()
}

fn main() {
    let input = read_input("2023", "12");

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
?###???????? 3,2,1
        "
        .trim();

        assert_eq!(part_1(input), "21");
    }

    #[ignore]
    #[test]
    fn test_part_2() {
        let input = r"".trim();

        assert_eq!(part_2(input), "");
    }
}
