use std::{collections::HashMap, time::Instant};

use aoc_rust::read_input;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

// solutions use Vecs instead of HashMaps because of very frequent lookups
// O(1) access in contiguous memory (Vec) wins over O(1) access in non-contiguous memory (HashMap)
// also HashMap has the overhead of .. you know, hashing
//
// just wanted a sub-1ms solution..

#[derive(Debug)]
struct Node<'a> {
    id: &'a str,
    left_idx: usize,
    right_idx: usize,
    // these are pretty much helpers as I couldn't figure out a better way
    // to build the nodes and their index links
    left_id: &'a str,
    right_id: &'a str,
}

fn nodes_from_str(s: &str) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    // helper hash map to build out `nodes`
    let mut indices: HashMap<&str, usize> = HashMap::new();

    for line in s.lines() {
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
    }

    for node in nodes.iter_mut() {
        node.left_idx = *indices.get(node.left_id).unwrap();
        node.right_idx = *indices.get(node.right_id).unwrap();
    }

    nodes
}

fn next_index(node: &Node, instruction: char) -> usize {
    match instruction {
        'L' => node.left_idx,
        'R' => node.right_idx,
        _ => 0,
    }
}

fn part_1(input: &str) -> String {
    let (instructions_str, nodes_str) = input.split_once("\n\n").unwrap();
    let mut instructions_iter = instructions_str.chars().cycle();
    let nodes = nodes_from_str(nodes_str);

    let mut iterations = 0;
    let mut idx = nodes
        .iter()
        .enumerate()
        .find(|(_, node)| node.id == "AAA")
        .unwrap()
        .0;

    while nodes[idx].id != "ZZZ" {
        iterations += 1;

        let instruction = instructions_iter.next().unwrap();

        idx = next_index(&nodes[idx], instruction);
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
    let nodes = nodes_from_str(nodes_str);

    nodes
        .iter()
        .enumerate()
        .filter_map(|(idx, node)| {
            if &node.id[2..] == "A" {
                Some(idx)
            } else {
                None
            }
        })
        .collect_vec()
        .par_iter()
        .map(|&idx| {
            let mut instructions_iter = instructions_str.chars().cycle();
            let mut cycle_start = 0;
            let mut cycle_end = 0;

            let mut current = idx;

            loop {
                cycle_end += 1;
                let instruction = instructions_iter.next().unwrap();

                current = match instruction {
                    'L' => nodes[current].left_idx,
                    'R' => nodes[current].right_idx,
                    _ => current,
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
