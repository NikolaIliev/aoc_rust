use std::{panic, time::Instant};

use aoc_rust::{
    direction::Direction, read_input, traversable_matrix::TraversableMatrix, uvec2::UVec2,
};

fn get_starting_position(m: &mut TraversableMatrix<char>) -> UVec2 {
    for y in 0..m.height {
        for x in 0..m.width {
            if m.matrix[y][x] == '^' {
                return UVec2 { x, y };
            }
        }
    }

    panic!("Could not find starting position");
}

fn get_rotated_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        _ => Direction::Up,
    }
}

fn get_distinct_visited_positions(m: &mut TraversableMatrix<char>) -> Vec<UVec2> {
    let mut direction = Direction::Up;
    let mut distinct_positions: Vec<UVec2> = vec![];

    let starting_position = get_starting_position(m);

    m.set_position(starting_position.x, starting_position.y);
    // mark starting position as visited
    m.matrix[m.position.y][m.position.x] = 'X';

    while let Some(ch) = m.peek_in_dir(direction) {
        if ch == '#' {
            direction = get_rotated_direction(direction);
            continue;
        }

        m.move_in_dir(direction);

        if ch == '.' {
            distinct_positions.push(UVec2 {
                x: m.position.x,
                y: m.position.y,
            });
        }

        m.matrix[m.position.y][m.position.x] = 'X';
    }

    return distinct_positions;
}

fn part_1(input: &str) -> String {
    let mut m = TraversableMatrix::<char>::from_str(input);

    // add 1 to include the starting position
    (get_distinct_visited_positions(&mut m).len() + 1).to_string()
}

fn find_loop(m: &mut TraversableMatrix<char>, visited_directions: &mut Vec<u8>) -> bool {
    let mut direction = Direction::Up;

    while let Some(next_pos) = m.pos_in_dir(direction) {
        let ch = m.matrix[next_pos.y][next_pos.x];

        if ch == '#' {
            direction = get_rotated_direction(direction);
            continue;
        }

        m.set_position(next_pos.x, next_pos.y);

        if visited_directions[m.position.y * m.width + m.position.x]
            & (2 as u8).pow(direction as u32)
            > 0
        {
            return true;
        }

        visited_directions[m.position.y * m.width + m.position.x] |=
            (2 as u8).pow(direction as u32);
    }

    return false;
}

fn part_2(input: &str) -> String {
    let mut m = TraversableMatrix::<char>::from_str(input);
    let mut visited_directions: Vec<u8> = vec![0; m.width * m.height];
    let mut loop_opportunities = 0;
    let starting_position = get_starting_position(&mut m);
    let distinct_positions = get_distinct_visited_positions(&mut m);

    for pos in distinct_positions {
        visited_directions.fill(0);
        m.set_position(starting_position.x, starting_position.y);
        m.matrix[pos.y][pos.x] = '#';

        if find_loop(&mut m, &mut visited_directions) {
            loop_opportunities += 1;
        }

        m.matrix[pos.y][pos.x] = '.'
    }

    return loop_opportunities.to_string();
}

fn main() {
    let input = read_input("2024", "06");

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
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
            "
        .trim();

        assert_eq!(part_1(input), "41");
    }

    #[test]
    fn test_part_2() {
        let input = r"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
            "
        .trim();

        assert_eq!(part_2(input), "6");
    }
}

