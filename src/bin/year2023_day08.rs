use std::{collections::HashMap, time::Instant};

use aoc_rust::read_input;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Default)]
struct Node {
    id: String,
    left: String,
    right: String,
}

fn part_1(input: &str) -> String {
    let (instructions_str, nodes_str) = input.split_once("\n\n").unwrap();
    let mut instructions_iter = instructions_str.chars().cycle();
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut iterations = 0;

    for line in nodes_str.lines() {
        let id = line[0..3].to_string();

        nodes.insert(
            id.to_owned(),
            Node {
                id,
                left: line[7..10].to_string(),
                right: line[12..15].to_string(),
            },
        );
    }

    let mut current = nodes.get("AAA").unwrap();

    while current.id != "ZZZ" {
        iterations += 1;

        let instruction = instructions_iter.next().unwrap();

        current = nodes
            .get(if instruction == 'L' {
                &current.left
            } else {
                &current.right
            })
            .unwrap();
    }

    iterations.to_string()
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }

    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn part_2(input: &str) -> String {
    let (instructions_str, nodes_str) = input.split_once("\n\n").unwrap();
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut starting_node_ids: Vec<String> = Vec::new();

    for line in nodes_str.lines() {
        let id = line[0..3].to_string();
        let id_last_ch = id.chars().nth(2).unwrap();
        let node = Node {
            id: line[0..3].to_string(),
            left: line[7..10].to_string(),
            right: line[12..15].to_string(),
        };

        nodes.insert(node.id.to_owned(), node);

        if id_last_ch == 'A' {
            starting_node_ids.push(id.to_owned());
        }
    }

    starting_node_ids
        .par_iter()
        .map(|id| {
            let mut instructions_iter = instructions_str.chars().cycle();
            let mut cycle_start = 0;
            let mut cycle_end = 0;

            let mut current = nodes.get(id).unwrap();

            loop {
                cycle_end += 1;
                let instruction = instructions_iter.next().unwrap();

                current = nodes
                    .get(if instruction == 'L' {
                        &current.left
                    } else {
                        &current.right
                    })
                    .unwrap();

                let last_char = *current.id.as_bytes().last().unwrap() as char;

                if last_char == 'Z' {
                    if cycle_start == 0 {
                        cycle_start = cycle_end;
                    } else {
                        return cycle_end - cycle_start;
                    }
                }
            }
        })
        .reduce_with(|a, b| lcm(a, b))
        .unwrap()
        .to_string()
}

fn main() {
    let input = read_input("2023", "08");

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
    fn test_part_1_1() {
        assert_eq!(
            part_1(
                r"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
                    .trim()
            ),
            "2"
        );
    }

    #[test]
    fn test_part_1_2() {
        assert_eq!(
            part_1(
                r"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
                    .trim()
            ),
            "6"
        );
    }

    #[test]
    fn test_part_2() {
        let input = r"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
        "
        .trim();

        assert_eq!(part_2(input), "6");
    }
}
