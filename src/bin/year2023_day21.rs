use std::{collections::HashSet, time::Instant};

use aoc_rust::read_input;
use itertools::Itertools;
use priority_queue::PriorityQueue;

fn parse_matrix(input: &str) -> (Vec<Vec<char>>, (isize, isize)) {
    let mut matrix = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if matrix[row][col] == 'S' {
                matrix[row][col] = '.';
                return (matrix, (col as isize, row as isize));
            }
        }
    }

    unreachable!()
}

fn can_go_to((x, y): (isize, isize), matrix: &Vec<Vec<char>>, visited: &Vec<Vec<usize>>) -> bool {
    x >= 0
        && y >= 0
        && (y as usize) < matrix.len()
        && (x as usize) < matrix[0].len()
        && visited[y as usize][x as usize] == usize::MAX
        && matrix[y as usize][x as usize] != '#'
}

fn part_1(input: &str, target_steps: usize) -> String {
    let (matrix, start) = parse_matrix(input);
    let mut visited = vec![vec![usize::MAX; matrix[0].len()]; matrix.len()];
    let mut reachable = vec![vec![false; matrix[0].len()]; matrix.len()];

    let mut pq = PriorityQueue::<(isize, isize), isize>::new();

    pq.push(start, 0);

    while !pq.is_empty() {
        let ((x, y), priority) = pq.pop().unwrap();
        let step = -priority;

        if step % 2 == (target_steps as isize) % 2 {
            reachable[y as usize][x as usize] = true;
        }

        if step as usize == target_steps {
            continue;
        }

        for pos in [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)] {
            if can_go_to(pos, &matrix, &visited) {
                visited[(pos.1) as usize][(pos.0) as usize] = (step + 1) as usize;

                pq.push(pos, priority - 1);
            }
        }
    }

    reachable
        .iter()
        .map(|row| row.iter().map(|&r| if r { 1 } else { 0 }).sum::<usize>())
        .sum::<usize>()
        .to_string()
}

fn count_positions(map: &Vec<Vec<char>>, start: (usize, usize), steps: usize) -> usize {
    let mut positions: HashSet<(usize, usize)> = HashSet::new();
    positions.insert(start);

    for _ in 0..steps {
        let mut new_positions: HashSet<(usize, usize)> = HashSet::new();
        for position in positions {
            let (y, x) = position;
            if y > 0 && map[y - 1][x] == '.' {
                new_positions.insert((y - 1, x));
            }
            if y < map.len() - 1 && map[y + 1][x] == '.' {
                new_positions.insert((y + 1, x));
            }
            if x > 0 && map[y][x - 1] == '.' {
                new_positions.insert((y, x - 1));
            }
            if x < map[y].len() - 1 && map[y][x + 1] == '.' {
                new_positions.insert((y, x + 1));
            }
        }
        positions = new_positions;
    }
    positions.len()
}

// https://www.youtube.com/watch?v=9UOMZSL0JTg
// so many assumptions based on input etc.. - did not like this at all, didn't bother after a certain point
// and just followed the above video
//
// 0/10 task for me tbh
fn part_2(input: &str) -> String {
    let (matrix, start_isize) = parse_matrix(input);
    let start = (start_isize.0 as usize, start_isize.1 as usize);

    let map_size = matrix.len();
    let grid_size = 26501365 / map_size - 1;

    let even_maps_in_grid = ((grid_size + 1) / 2 * 2).pow(2);
    let odd_maps_in_grid = (grid_size / 2 * 2 + 1).pow(2);

    let odd_points_in_map = count_positions(&matrix, start, map_size * 2 + 1);
    let even_points_in_map = count_positions(&matrix, start, map_size * 2);

    let total_points_fully_in_grid =
        odd_points_in_map * odd_maps_in_grid + even_points_in_map * even_maps_in_grid;

    let corner_top = count_positions(&matrix, (map_size - 1, start.1), map_size - 1);
    let corner_right = count_positions(&matrix, (start.0, 0), map_size - 1);
    let corner_bottom = count_positions(&matrix, (0, start.1), map_size - 1);
    let corner_left = count_positions(&matrix, (start.0, map_size - 1), map_size - 1);

    let total_points_in_grid_corners = corner_top + corner_right + corner_bottom + corner_left;

    let small_diag_top_right = count_positions(&matrix, (map_size - 1, 0), map_size / 2 - 1);
    let small_diag_bottom_right = count_positions(&matrix, (0, 0), map_size / 2 - 1);
    let small_diag_bottom_left = count_positions(&matrix, (0, map_size - 1), map_size / 2 - 1);
    let small_diag_top_left =
        count_positions(&matrix, (map_size - 1, map_size - 1), map_size / 2 - 1);

    let total_points_in_small_diags = (grid_size + 1)
        * (small_diag_top_right
            + small_diag_bottom_right
            + small_diag_bottom_left
            + small_diag_top_left);

    let big_diag_top_right = count_positions(&matrix, (map_size - 1, 0), map_size * 3 / 2 - 1);
    let big_diag_bottom_right = count_positions(&matrix, (0, 0), map_size * 3 / 2 - 1);
    let big_diag_bottom_left = count_positions(&matrix, (0, map_size - 1), map_size * 3 / 2 - 1);
    let big_diag_top_left =
        count_positions(&matrix, (map_size - 1, map_size - 1), map_size * 3 / 2 - 1);

    let total_points_in_big_diags = grid_size
        * (big_diag_top_right + big_diag_bottom_right + big_diag_bottom_left + big_diag_top_left);

    let total_points_in_diag = total_points_in_small_diags + total_points_in_big_diags;

    (total_points_fully_in_grid + total_points_in_grid_corners + total_points_in_diag).to_string()
}

fn main() {
    let input = read_input("2023", "21");

    let start_part_1 = Instant::now();
    let part_1_result = part_1(&input, 64);
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
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
        "
        .trim();

        assert_eq!(part_1(input, 6), "16");
    }
}
