use std::{collections::HashMap, time::Instant};

use aoc_rust::read_input;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Default)]
struct Node<'a> {
    id: &'a str,
    left_id: &'a str,
    right_id: &'a str,
    left_idx: usize,
    right_idx: usize,
}

fn part_1<'a>(input: &'a str) -> String {
    let (instructions_str, nodes_str) = input.split_once("\n\n").unwrap();
    let mut instructions_iter = instructions_str.chars().cycle();
    let mut nodes: Vec<Node> = Vec::new();
    let mut indices: HashMap<&'a str, usize> = HashMap::new();
    let mut iterations = 0;
    let mut current = 0;

    for line in nodes_str.lines() {
        let idx = nodes.len();
        let id = &line[0..3];
        let left_id = &line[7..10];
        let right_id = &line[12..15];

        nodes.push(Node {
            id,
            left_id,
            right_id,
            left_idx: 0,
            right_idx: 0,
        });
        indices.insert(id, idx);

        if id == "AAA" {
            current = idx;
        }
    }

    for node in nodes.iter_mut() {
        node.left_idx = *indices.get(node.left_id).unwrap();
        node.right_idx = *indices.get(node.right_id).unwrap();
    }

    while nodes[current].id != "ZZZ" {
        iterations += 1;

        let instruction = instructions_iter.next().unwrap();

        current = if instruction == 'L' {
            nodes[current].left_idx
        } else {
            nodes[current].right_idx
        };
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

fn part_2<'a>(input: &'a str) -> String {
    let (instructions_str, nodes_str) = input.split_once("\n\n").unwrap();
    let mut nodes: Vec<Node> = Vec::new();
    let mut starting_node_indexes: Vec<usize> = Vec::new();
    let mut indices: HashMap<&'a str, usize> = HashMap::new();

    for line in nodes_str.lines() {
        let idx = nodes.len();
        let id = &line[0..3];
        let left_id = &line[7..10];
        let right_id = &line[12..15];

        nodes.push(Node {
            id,
            left_id,
            right_id,
            left_idx: 0,
            right_idx: 0,
        });
        indices.insert(id, idx);

        if &id[2..] == "A" {
            starting_node_indexes.push(idx);
        }
    }

    for node in nodes.iter_mut() {
        node.left_idx = *indices.get(node.left_id).unwrap();
        node.right_idx = *indices.get(node.right_id).unwrap();
    }

    starting_node_indexes
        .par_iter()
        .map(|&idx| {
            let mut instructions_iter = instructions_str.chars().cycle();
            let mut cycle_start = 0;
            let mut cycle_end = 0;

            let mut current = idx;

            loop {
                cycle_end += 1;
                let instruction = instructions_iter.next().unwrap();

                current = if instruction == 'L' {
                    nodes[current].left_idx
                } else {
                    nodes[current].right_idx
                };

                let last_char = *nodes[current].id.as_bytes().last().unwrap() as char;

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
