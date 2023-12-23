use std::{collections::HashSet, time::Instant};

use aoc_rust::read_input;
use itertools::Itertools;
use priority_queue::PriorityQueue;

fn parse_matrix(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

fn can_move_to(matrix: &Vec<Vec<char>>, visited: &Vec<Vec<bool>>, (x, y): (isize, isize)) -> bool {
    x >= 0
        && y >= 0
        && (x as usize) < matrix[0].len()
        && (y as usize) < matrix.len()
        && !visited[y as usize][x as usize]
        && matrix[y as usize][x as usize] != '#'
}

fn resolve_pos(
    matrix: &Vec<Vec<char>>,
    visited: &Vec<Vec<bool>>,
    (x, y): (isize, isize),
    steps: usize,
    slippery_slopes: bool,
) -> Option<((isize, isize), usize)> {
    if !can_move_to(matrix, visited, (x, y)) {
        None
    } else {
        if slippery_slopes {
            match matrix[y as usize][x as usize] {
                '>' => {
                    if can_move_to(matrix, visited, (x + 1, y)) {
                        Some(((x + 1, y), steps + 2))
                    } else {
                        None
                    }
                }
                'v' => {
                    if can_move_to(matrix, visited, (x, y + 1)) {
                        Some(((x, y + 1), steps + 2))
                    } else {
                        None
                    }
                }
                '^' => {
                    if can_move_to(matrix, visited, (x, y - 1)) {
                        Some(((x, y - 1), steps + 2))
                    } else {
                        None
                    }
                }
                '<' => {
                    if can_move_to(matrix, visited, (x - 1, y)) {
                        Some(((x - 1, y), steps + 2))
                    } else {
                        None
                    }
                }
                _ => Some(((x, y), steps + 1)),
            }
        } else {
            Some(((x, y), steps + 1))
        }
    }
}

fn manhattan_distance((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> isize {
    (x1.abs_diff(x2) + y1.abs_diff(y2)) as isize
}

fn part_1(input: &str) -> String {
    let matrix = parse_matrix(input);
    let end = ((matrix[0].len() - 2) as isize, (matrix.len() - 1) as isize);
    let mut max_steps_at: Vec<Vec<usize>> = vec![vec![0; matrix[0].len()]; matrix.len()];

    let mut pq = PriorityQueue::<((isize, isize), usize, Vec<Vec<bool>>), isize>::new();

    pq.push(
        ((1, 0), 0, vec![vec![false; matrix[0].len()]; matrix.len()]),
        0,
    );

    let mut max_steps = 0;

    while !pq.is_empty() {
        let (((x, y), steps, mut visited), _) = pq.pop().unwrap();

        if max_steps_at[y as usize][x as usize] > steps {
            continue;
        }

        max_steps_at[y as usize][x as usize] = steps;

        if x == end.0 && y == end.1 {
            max_steps = max_steps.max(steps);
        }

        visited[y as usize][x as usize] = true;

        let next = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
            .iter()
            .filter_map(|&potential_next_pos| {
                resolve_pos(&matrix, &visited, potential_next_pos, steps, true)
            })
            .collect_vec();

        match next.len() {
            0 => {}
            1 => {
                let (next_pos, next_steps) = next[0];

                pq.push(
                    (next_pos, next_steps, visited),
                    manhattan_distance(next_pos, end) * (next_steps) as isize,
                );
            }
            _ => {
                for &(next_pos, next_steps) in next.iter() {
                    pq.push(
                        (next_pos, next_steps, visited.clone()),
                        manhattan_distance(next_pos, end) * (next_steps) as isize,
                    );
                }
            }
        }
        {}
    }

    max_steps.to_string()
}

fn part_2(input: &str) -> String {
    let matrix = parse_matrix(input);
    let end = ((matrix[0].len() - 2) as isize, (matrix.len() - 1) as isize);
    let mut shortcuts: Vec<Vec<HashSet<((isize, isize), usize)>>> =
        vec![vec![HashSet::new(); matrix[0].len()]; matrix.len()];

    let visited = vec![vec![false; matrix[0].len()]; matrix.len()];

    for m_y in 0..matrix.len() as isize {
        for m_x in 0..matrix[m_y as usize].len() as isize {
            if matrix[m_y as usize][m_x as usize] != '#' {
                let possible_connections = [
                    (m_x + 1, m_y),
                    (m_x - 1, m_y),
                    (m_x, m_y + 1),
                    (m_x, m_y - 1),
                ];
                let connections = possible_connections
                    .iter()
                    .filter(|&potential_next_pos| {
                        can_move_to(
                            &matrix,
                            &visited,
                            (potential_next_pos.0, potential_next_pos.1),
                        )
                    })
                    .collect_vec();

                if (m_x == 1 && m_y == 0) || connections.len() > 2 {
                    for (mc_x, mc_y) in connections.iter() {
                        let mut pq: PriorityQueue<((isize, isize), usize), usize> =
                            PriorityQueue::new();
                        let mut visited = vec![vec![false; matrix[0].len()]; matrix.len()];

                        visited[m_y as usize][m_x as usize] = true;
                        pq.push(((*mc_x, *mc_y), 1), 0);

                        while !pq.is_empty() {
                            let (((x, y), steps), _) = pq.pop().unwrap();

                            visited[y as usize][x as usize] = true;

                            if x == end.0 && y == end.1 {
                                shortcuts[m_y as usize][m_x as usize].insert((end, steps));
                                shortcuts[end.1 as usize][end.0 as usize]
                                    .insert(((m_x, m_y), steps));

                                continue;
                            }

                            let potential_next_pos =
                                [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
                            let next = potential_next_pos
                                .iter()
                                .filter(|potential_next_pos| {
                                    can_move_to(
                                        &matrix,
                                        &visited,
                                        (potential_next_pos.0, potential_next_pos.1),
                                    )
                                })
                                .collect_vec();

                            if next.is_empty() {
                                continue;
                            }

                            if next.len() > 1 {
                                shortcuts[m_y as usize][m_x as usize].insert(((x, y), steps));
                                shortcuts[y as usize][x as usize].insert(((m_x, m_y), steps));
                            } else {
                                pq.push((*next[0], steps + 1), 0);
                            }
                        }
                    }
                }
            }
        }
    }

    let mut graph_intermediary_helper: Vec<(usize, usize)> = vec![];

    for y in 0..shortcuts.len() {
        for x in 0..shortcuts[y].len() {
            let paths = &shortcuts[y][x];

            if !paths.is_empty() {
                graph_intermediary_helper.push((x, y));
                //println!("Shortcuts starting from ({x}, {y}): {:?}", paths);
            }
        }
    }

    let mut graph: Vec<Vec<(usize, usize)>> = vec![vec![]; graph_intermediary_helper.len()];

    for i in 0..graph_intermediary_helper.len() {
        for &((x, y), steps) in
            &shortcuts[graph_intermediary_helper[i].1][graph_intermediary_helper[i].0]
        {
            let connection_idx = graph_intermediary_helper
                .iter()
                .find_position(|a| a.0 as isize == x && a.1 as isize == y)
                .unwrap()
                .0;

            graph[i].push((connection_idx, steps));
        }
    }

    let mut max_steps = 0;

    let mut pq: Vec<(usize, usize, usize)> = vec![(0, 0, 0)];

    while !pq.is_empty() {
        let (idx, steps, visited) = pq.pop().unwrap();

        if idx == graph.len() - 1 {
            max_steps = max_steps.max(steps);
            continue;
        }

        for &(next_idx, next_steps) in &graph[idx] {
            if visited & (1 << next_idx) == 0 {
                let new_visited = visited | (1 << next_idx);
                pq.push((next_idx, steps + next_steps, new_visited));
            }
        }
    }

    max_steps.to_string()
}

fn main() {
    let input = read_input("2023", "23");

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
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"
            .trim();

        assert_eq!(part_1(input), "94");
    }

    #[test]
    fn test_part_2() {
        let input = r"
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"
            .trim();

        assert_eq!(part_2(input), "154");
    }
}
