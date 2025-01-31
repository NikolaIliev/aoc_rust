use std::{collections::HashMap, time::Instant};

use aoc_rust::read_input;
use itertools::Itertools;

#[derive(Debug)]
enum ConditionType {
    LessThan,
    GreaterThan,
}

#[derive(Debug)]
struct Condition {
    condition_type: ConditionType,
    left: char,
    right: usize,
}

#[derive(Debug)]
struct Node<'a> {
    condition: Option<Condition>,
    then: &'a str,
}

#[derive(Debug)]
struct Workflow<'a> {
    nodes: Vec<Node<'a>>,
}

#[derive(Debug, Default)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

fn parse_workflows<'a>(workflows_str: &'a str) -> HashMap<&'a str, Workflow<'a>> {
    workflows_str.lines().fold(
        HashMap::<&str, Workflow<'a>>::new(),
        |mut workflows, line| {
            let (id, nodes) = line.split_once("{").unwrap();

            workflows.insert(
                id,
                Workflow {
                    nodes: nodes[..nodes.len() - 1]
                        .split(",")
                        .map(|node_str| {
                            if let Some((cond_str, then)) = node_str.split_once(":") {
                                Node {
                                    condition: Some(Condition {
                                        condition_type: match cond_str.chars().nth(1).unwrap() {
                                            '>' => ConditionType::GreaterThan,
                                            '<' => ConditionType::LessThan,
                                            _ => unreachable!(),
                                        },
                                        left: cond_str.chars().next().unwrap(),
                                        right: *&cond_str[2..].parse::<usize>().unwrap(),
                                    }),
                                    then,
                                }
                            } else {
                                Node {
                                    condition: None,
                                    then: node_str,
                                }
                            }
                        })
                        .collect_vec(),
                },
            );

            workflows
        },
    )
}

fn parse_parts(parts_str: &str) -> Vec<Part> {
    parts_str
        .lines()
        .map(|line| {
            line[1..line.len() - 1].split(",").enumerate().fold(
                Part {
                    x: 0,
                    m: 0,
                    a: 0,
                    s: 0,
                },
                |mut part, (idx, part_prop_str)| {
                    let value = part_prop_str
                        .split_once('=')
                        .unwrap()
                        .1
                        .parse::<usize>()
                        .unwrap();

                    match idx {
                        0 => part.x = value,
                        1 => part.m = value,
                        2 => part.a = value,
                        3 => part.s = value,
                        _ => unreachable!(),
                    };

                    part
                },
            )
        })
        .collect_vec()
}

fn part_1(input: &str) -> String {
    let (workflows_str, parts_str) = input.split_once("\n\n").unwrap();
    let workflows = parse_workflows(workflows_str);
    let parts = parse_parts(parts_str);

    parts
        .iter()
        .filter_map(|part| -> Option<usize> {
            let mut current_workflow = workflows.get("in").unwrap();

            loop {
                for node in &current_workflow.nodes {
                    let then = if let Some(condition) = &node.condition {
                        let left_operand = match condition.left {
                            'x' => part.x,
                            'm' => part.m,
                            'a' => part.a,
                            's' => part.s,
                            _ => unreachable!(),
                        };

                        if match condition.condition_type {
                            ConditionType::GreaterThan => left_operand > condition.right,
                            ConditionType::LessThan => left_operand < condition.right,
                        } {
                            node.then
                        } else {
                            continue;
                        }
                    } else {
                        node.then
                    };

                    match then {
                        "R" => return None,
                        "A" => return Some(part.x + part.m + part.a + part.s),
                        workflow_id => {
                            current_workflow = workflows.get(workflow_id).unwrap();
                            break;
                        }
                    }
                }
            }
        })
        .sum::<usize>()
        .to_string()
}

fn get_distinct_accepted_parts_count<'a>(
    workflows: &HashMap<&str, Workflow<'a>>,
    workflow_id: &str,
    ranges: (
        (usize, usize),
        (usize, usize),
        (usize, usize),
        (usize, usize),
    ),
) -> usize {
    let nodes = &workflows.get(workflow_id).unwrap().nodes;

    let mut workflow_ranges = ranges.clone();

    nodes
        .iter()
        // go through each node ->
        //
        //  if there's a condition ->
        //
        //    create new ranges for when it passes and start the appropriate workflow or return a sum
        //    of distinct values if "then" is "A" (accepted)
        //
        //    when it doesn't pass, just mutate workflow_ranges so the next node works with the
        //    cut-off ranges
        //
        //  if no condition ->
        //    handle "then" with the current workflow_ranges
        .map(|node| {
            let mut matched_condition_ranges = workflow_ranges.clone();

            if let Some(condition) = &node.condition {
                let matched_condition_range_to_change = match condition.left {
                    'x' => &mut matched_condition_ranges.0,
                    'm' => &mut matched_condition_ranges.1,
                    'a' => &mut matched_condition_ranges.2,
                    's' => &mut matched_condition_ranges.3,
                    _ => unreachable!(),
                };
                let workflow_range_to_change = match condition.left {
                    'x' => &mut workflow_ranges.0,
                    'm' => &mut workflow_ranges.1,
                    'a' => &mut workflow_ranges.2,
                    's' => &mut workflow_ranges.3,
                    _ => unreachable!(),
                };

                match condition.condition_type {
                    ConditionType::GreaterThan => {
                        matched_condition_range_to_change.0 = condition.right + 1;
                        workflow_range_to_change.1 = condition.right;
                    }
                    ConditionType::LessThan => {
                        matched_condition_range_to_change.1 = condition.right - 1;
                        workflow_range_to_change.0 = condition.right;
                    }
                };
            }

            match node.then {
                "A" => {
                    // accepted -> calc all permutations based on the mutated ranges in matched_condition_ranges
                    (matched_condition_ranges.0 .1 - matched_condition_ranges.0 .0 + 1)
                        * (matched_condition_ranges.1 .1 - matched_condition_ranges.1 .0 + 1)
                        * (matched_condition_ranges.2 .1 - matched_condition_ranges.2 .0 + 1)
                        * (matched_condition_ranges.3 .1 - matched_condition_ranges.3 .0 + 1)
                }
                "R" => 0,
                // go to next workflow with the mutated matched_condition_ranges
                workflow_id => get_distinct_accepted_parts_count(
                    workflows,
                    workflow_id,
                    matched_condition_ranges,
                ),
            }
        })
        .sum::<usize>()
}

fn part_2(input: &str) -> String {
    let workflows = parse_workflows(input.split_once("\n\n").unwrap().0);

    get_distinct_accepted_parts_count(
        &workflows,
        "in",
        ((1, 4000), (1, 4000), (1, 4000), (1, 4000)),
    )
    .to_string()
}

fn main() {
    let input = read_input("2023", "19");

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
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
        "
        .trim();

        assert_eq!(part_1(input), "19114");
    }

    #[test]
    fn test_part_2() {
        let input = r"
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
        "
        .trim();

        assert_eq!(part_2(input), "167409079868000");
    }
}
