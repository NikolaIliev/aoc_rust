use std::time::Instant;

use aoc_rust::read_input;
use itertools::Itertools;

fn parse_matrix(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

fn bottom_right(matrix: &Vec<Vec<char>>) -> (isize, isize) {
    ((matrix[0].len() - 1) as isize, (matrix.len() - 1) as isize)
}

fn hash_location(matrix: &Vec<Vec<char>>, location: (isize, isize)) -> usize {
    matrix.len() * (location.1 as usize) + (location.0 as usize)
}

fn hash_location_with_direction(
    matrix: &Vec<Vec<char>>,
    location: (isize, isize),
    direction: (isize, isize),
) -> usize {
    hash_location(matrix, location)
        + match direction {
            (1, 0) => 2,
            (-1, 0) => 3,
            (0, 1) => 4,
            (0, -1) => 5,
            _ => panic!(),
        } * hash_location(matrix, bottom_right(matrix))
}

fn beam(
    matrix: &mut Vec<Vec<char>>,
    (x, y): (isize, isize),
    direction: (isize, isize),
    visited: &mut Vec<usize>,
    visited_with_direction: &mut Vec<usize>,
) {
    if x < 0
        || y < 0
        || y as usize >= matrix.len()
        || x as usize >= matrix[0].len()
        // already been here with the same direction, stop
        || visited_with_direction[hash_location_with_direction(matrix, (x, y), direction)] == 1
    {
        return;
    }

    let ch = matrix[y as usize][x as usize];

    visited[hash_location(matrix, (x, y))] = 1;
    visited_with_direction[hash_location_with_direction(matrix, (x, y), direction)] = 1;

    match ch {
        '.' | '#' => beam(
            matrix,
            (x + direction.0, y + direction.1),
            direction,
            visited,
            visited_with_direction,
        ),
        '/' => {
            let new_direction = (-direction.1, -direction.0);

            beam(
                matrix,
                (x + new_direction.0, y + new_direction.1),
                new_direction,
                visited,
                visited_with_direction,
            )
        }
        '\\' => {
            let new_direction = (direction.1, direction.0);

            beam(
                matrix,
                (x + new_direction.0, y + new_direction.1),
                new_direction,
                visited,
                visited_with_direction,
            )
        }
        '|' => match direction {
            (0, 1) | (0, -1) => beam(
                matrix,
                (x + direction.0, y + direction.1),
                direction,
                visited,
                visited_with_direction,
            ),
            _ => {
                beam(matrix, (x, y + 1), (0, 1), visited, visited_with_direction);
                beam(matrix, (x, y - 1), (0, -1), visited, visited_with_direction);
            }
        },
        '-' => match direction {
            (1, 0) | (-1, 0) => beam(
                matrix,
                (x + direction.0, y + direction.1),
                direction,
                visited,
                visited_with_direction,
            ),
            _ => {
                beam(matrix, (x + 1, y), (1, 0), visited, visited_with_direction);
                beam(matrix, (x - 1, y), (-1, 0), visited, visited_with_direction);
            }
        },
        _ => panic!(),
    }
}

fn part_1(input: &str) -> String {
    let mut matrix = parse_matrix(input);
    let width = matrix[0].len();
    let height = matrix.len();
    let max_pos = ((width - 1) as isize, (height - 1) as isize);

    let mut visited: Vec<usize> = vec![0; hash_location(&matrix, max_pos) + 1];
    let mut visited_with_direction: Vec<usize> = vec![
        0;
        hash_location_with_direction(
            &matrix,
            max_pos,
            // location with largest multiplier
            (0, -1)
        ) + 1
    ];

    beam(
        &mut matrix,
        (0, 0),
        (1, 0),
        &mut visited,
        &mut visited_with_direction,
    );

    visited.iter().filter(|&x| *x == 1).count().to_string()
}

fn part_2(input: &str) -> String {
    let mut matrix = parse_matrix(input);
    let width = matrix[0].len();
    let height = matrix.len();
    let max_pos = ((width - 1) as isize, (height - 1) as isize);

    let mut visited: Vec<usize> = vec![0; hash_location(&matrix, max_pos) + 1];
    let mut visited_with_direction: Vec<usize> = vec![
        0;
        hash_location_with_direction(
            &matrix,
            max_pos,
            // location with largest multiplier
            (0, -1)
        ) + 1
    ];

    let mut max = 0;

    // from left
    for y in 0..height {
        visited.fill(0);
        visited_with_direction.fill(0);
        beam(
            &mut matrix,
            (0, y as isize),
            (1, 0),
            &mut visited,
            &mut visited_with_direction,
        );
        let score = visited.iter().filter(|&x| *x == 1).count();

        if score > max {
            max = score;
        }
    }

    // from top
    for x in 0..width {
        visited.fill(0);
        visited_with_direction.fill(0);
        beam(
            &mut matrix,
            (x as isize, 0),
            (0, 1),
            &mut visited,
            &mut visited_with_direction,
        );
        let score = visited.iter().filter(|&x| *x == 1).count();

        if score > max {
            max = score;
        }
    }

    // from right
    for y in 0..height {
        visited.fill(0);
        visited_with_direction.fill(0);
        beam(
            &mut matrix,
            ((width - 1) as isize, y as isize),
            (-1, 0),
            &mut visited,
            &mut visited_with_direction,
        );
        let score = visited.iter().filter(|&x| *x == 1).count();

        if score > max {
            max = score;
        }
    }

    // from bottom
    for x in 0..width {
        visited.fill(0);
        visited_with_direction.fill(0);
        beam(
            &mut matrix,
            (x as isize, (height - 1) as isize),
            (0, -1),
            &mut visited,
            &mut visited_with_direction,
        );
        let score = visited.iter().filter(|&x| *x == 1).count();

        if score > max {
            max = score;
        }
    }

    max.to_string()
}

fn main() {
    let input = read_input("2023", "16");

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
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
        "
        .trim();

        assert_eq!(part_1(input), "46");
    }

    #[ignore]
    #[test]
    fn test_part_2() {
        let input = r"".trim();

        assert_eq!(part_2(input), "");
    }
}
