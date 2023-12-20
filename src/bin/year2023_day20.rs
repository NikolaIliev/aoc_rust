use std::{collections::VecDeque, time::Instant};

use aoc_rust::read_input;
use itertools::Itertools;

const BROADCAST_MODULE_HASHED_ID: usize = 0;

#[derive(Debug, Clone)]
enum Module {
    Broadcaster(Vec<usize>),
    FlipFlop((Vec<usize>, bool)),
    Conjuction((Vec<usize>, Vec<(usize, bool)>)),
}

impl Module {
    fn receive_pulse(
        &mut self,
        input_idx: usize,
        high: bool,
    ) -> Box<dyn Iterator<Item = (usize, bool)> + '_> {
        match self {
            Module::Broadcaster(outputs) => {
                Box::new(outputs.iter().map(move |&output| (output, false)))
            }
            Module::FlipFlop((outputs, on)) => {
                if high {
                    Box::new(std::iter::empty())
                } else {
                    *on = !*on;
                    Box::new(outputs.iter().map(move |&output| (output, *on)))
                }
            }
            Module::Conjuction((outputs, state)) => {
                for (idx, remembered_high) in state.iter_mut() {
                    if *idx == input_idx {
                        *remembered_high = high;
                    }
                }

                let high = state.iter().any(|&(_, high)| !high);
                Box::new(outputs.iter().map(move |&output| (output, high)))
            }
        }
    }
}

fn hash_module_id(id: &str) -> usize {
    if id == "broadcaster" {
        BROADCAST_MODULE_HASHED_ID
    } else {
        id.chars().enumerate().fold(0, |sum, (idx, ch)| {
            sum + (b'z').pow(idx as u32) as usize * (ch as u8 - b'a') as usize
        })
    }
}

fn parse_modules(input: &str) -> Vec<Module> {
    let max_hashed_id = input
        .lines()
        .map(|line| {
            let (type_and_id_str, _) = line.split_once(" -> ").unwrap();

            let hashed = hash_module_id(if type_and_id_str == "broadcaster" {
                type_and_id_str
            } else {
                &type_and_id_str[1..]
            });

            hashed
        })
        .max()
        .unwrap();

    let mut modules: Vec<Module> = vec![Module::Broadcaster(vec![]); max_hashed_id + 1];

    for line in input.lines() {
        let (type_and_id_str, outputs_str) = line.split_once(" -> ").unwrap();

        let outputs = outputs_str.split(", ").map(hash_module_id).collect_vec();

        match type_and_id_str {
            "broadcaster" => modules[BROADCAST_MODULE_HASHED_ID] = Module::Broadcaster(outputs),
            _ => {
                let hashed_id = hash_module_id(&type_and_id_str[1..]);

                match type_and_id_str.chars().next().unwrap() {
                    '%' => modules[hashed_id] = Module::FlipFlop((outputs, false)),
                    '&' => modules[hashed_id] = Module::Conjuction((outputs, vec![])),
                    _ => unreachable!(),
                }
            }
        };
    }

    // populate conjuction inputs
    for line in input.lines() {
        let (type_and_id_str, outputs_str) = line.split_once(" -> ").unwrap();

        let id = if type_and_id_str == "broadcaster" {
            type_and_id_str
        } else {
            &type_and_id_str[1..]
        };

        for output_id in outputs_str.split(", ") {
            if let Some(module) = modules.get_mut(hash_module_id(output_id)) {
                if let Module::Conjuction((_, state)) = module {
                    state.push((hash_module_id(id), false))
                };
            }
        }
    }

    modules
}

fn part_1(input: &str) -> String {
    let mut modules = parse_modules(input);
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    let mut queue = VecDeque::<(usize, usize, bool)>::new();

    for _ in 1..=1000 {
        queue.push_back((42, BROADCAST_MODULE_HASHED_ID, false));

        while let Some((input, output, high)) = queue.pop_front() {
            match high {
                true => high_pulses += 1,
                false => low_pulses += 1,
            };

            if let Some(module) = modules.get_mut(output) {
                for (next, high) in module.receive_pulse(input, high) {
                    queue.push_back((output, next, high));
                }
            }
        }
    }

    return (low_pulses * high_pulses).to_string();
}

fn part_2(input: &str) -> String {
    let mut modules = parse_modules(input);

    let mut button_presses = 0;
    let mut queue = VecDeque::<(usize, usize, bool)>::new();
    // the id of the Conjuction module that feeds into "rx"
    let rx_hashed = hash_module_id("rx");
    let rx_input = modules
        .iter()
        .enumerate()
        .find_map(|(idx, module)| {
            if let Module::Conjuction((outputs, _)) = module {
                if outputs.iter().any(|&idx| idx == rx_hashed) {
                    Some(idx)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap();

    // the ids of the conjuction modules that feed into the conjuction module that feeds into rx
    // all of these must send a high pulse so that the "rx_input" module will fire a Low pulse at
    // rx
    let linked_conjuction_modules: Vec<usize> =
        if let Module::Conjuction((_, state)) = &mut modules[rx_input] {
            state.iter().map(|(id, _)| *id).collect_vec()
        } else {
            vec![]
        };

    // fortunately they all fire a high pulse periodically and the loop starts at 0
    // so we just find the first time each fires a high pulse, and then multiply those numbers
    // together to find the first time they'll all fire a high pulse at the same time...
    let mut first_sent_high_pulse_at: Vec<usize> = vec![0; modules.len()];

    loop {
        queue.push_back((42, BROADCAST_MODULE_HASHED_ID, false));
        button_presses += 1;

        while let Some((input, output, high)) = queue.pop_front() {
            if high && first_sent_high_pulse_at[input] == 0 {
                first_sent_high_pulse_at[input] = button_presses;
            }

            if let Some(module) = modules.get_mut(output) {
                for (next, high) in module.receive_pulse(input, high) {
                    queue.push_back((output, next, high));
                }
            }
        }

        if !linked_conjuction_modules
            .iter()
            .any(|&idx| first_sent_high_pulse_at[idx] == 0)
        {
            return linked_conjuction_modules
                .iter()
                .map(|&idx| first_sent_high_pulse_at[idx])
                .product::<usize>()
                .to_string();
        }
    }
}

fn main() {
    let input = read_input("2023", "20");

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
    fn test_part_1_1() {
        let input = r"
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
        "
        .trim();

        assert_eq!(part_1(input), "32000000");
    }

    #[test]
    fn test_part_1_2() {
        let input = r"
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
        "
        .trim();

        assert_eq!(part_1(input), "11687500");
    }

    #[ignore]
    #[test]
    fn test_part_2() {
        let input = r"".trim();

        assert_eq!(part_2(input), "");
    }
}
