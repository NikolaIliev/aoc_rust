use std::{fmt::Debug, str::FromStr, time::Instant};

use aoc_rust::read_input;
use itertools::Itertools;

enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl Direction {
    fn all() -> &'static [Direction] {
        &[
            Direction::Right,
            Direction::Left,
            Direction::Down,
            Direction::Up,
            Direction::UpRight,
            Direction::UpLeft,
            Direction::DownRight,
            Direction::DownLeft,
        ]
    }
}

#[derive(Debug)]
struct UVec2 {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct TraversableMatrix<T>
where
    T: FromStr,
{
    matrix: Vec<Vec<T>>,
    width: usize,
    height: usize,
    position: UVec2,
}

impl<T> TraversableMatrix<T>
where
    T: FromStr + Copy,
    <T as FromStr>::Err: Debug,
{
    fn from_str(s: &str) -> TraversableMatrix<T> {
        let matrix = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| T::from_str(&ch.to_string()).unwrap())
                    .collect_vec()
            })
            .collect_vec();

        TraversableMatrix::<T> {
            width: matrix[0].len(),
            height: matrix.len(),
            matrix,
            position: UVec2 { x: 0, y: 0 },
        }
    }

    fn cur(&self) -> T {
        return self.matrix[self.position.y][self.position.x];
    }

    fn set_position(&mut self, x: usize, y: usize) {
        if y >= self.height || x >= self.width {
            return;
        }

        self.position.x = x;
        self.position.y = y;
    }

    fn can_move_up(&self) -> bool {
        self.position.y > 0
    }

    fn can_move_right(&self) -> bool {
        self.position.x < self.width - 1
    }

    fn can_move_down(&self) -> bool {
        self.position.y < self.height - 1
    }

    fn can_move_left(&self) -> bool {
        self.position.x > 0
    }

    fn peek_in_dir(&self, direction: &Direction) -> Option<T> {
        let UVec2 { x, y } = self.position;

        match direction {
            Direction::Up => {
                if self.can_move_up() {
                    Some(self.matrix[y - 1][x])
                } else {
                    None
                }
            }
            Direction::Right => {
                if self.can_move_right() {
                    Some(self.matrix[y][x + 1])
                } else {
                    None
                }
            }
            Direction::Down => {
                if self.can_move_down() {
                    Some(self.matrix[y + 1][x])
                } else {
                    None
                }
            }
            Direction::Left => {
                if self.can_move_left() {
                    Some(self.matrix[y][x - 1])
                } else {
                    None
                }
            }
            Direction::UpLeft => {
                if self.can_move_up() && self.can_move_left() {
                    Some(self.matrix[y - 1][x - 1])
                } else {
                    None
                }
            }
            Direction::UpRight => {
                if self.can_move_up() && self.can_move_right() {
                    Some(self.matrix[y - 1][x + 1])
                } else {
                    None
                }
            }
            Direction::DownLeft => {
                if self.can_move_down() && self.can_move_left() {
                    Some(self.matrix[y + 1][x - 1])
                } else {
                    None
                }
            }
            Direction::DownRight => {
                if self.can_move_down() && self.can_move_right() {
                    Some(self.matrix[y + 1][x + 1])
                } else {
                    None
                }
            }
        }
    }

    fn move_in_dir(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => {
                if self.can_move_up() {
                    self.position.y -= 1;
                }
            }
            Direction::Right => {
                if self.can_move_right() {
                    self.position.x += 1;
                }
            }
            Direction::Down => {
                if self.can_move_down() {
                    self.position.y += 1;
                }
            }
            Direction::Left => {
                if self.can_move_left() {
                    self.position.x -= 1;
                }
            }
            Direction::UpLeft => {
                if self.can_move_up() && self.can_move_left() {
                    self.position.y -= 1;
                    self.position.x -= 1;
                }
            }
            Direction::UpRight => {
                if self.can_move_up() && self.can_move_right() {
                    self.position.y -= 1;
                    self.position.x += 1;
                }
            }
            Direction::DownLeft => {
                if self.can_move_down() && self.can_move_left() {
                    self.position.y += 1;
                    self.position.x -= 1;
                }
            }
            Direction::DownRight => {
                if self.can_move_down() && self.can_move_right() {
                    self.position.y += 1;
                    self.position.x += 1;
                }
            }
        }
    }
}

const TARGET_LETTERS: &[char] = &['X', 'M', 'A', 'S'];

fn part_1(input: &str) -> String {
    let mut m = TraversableMatrix::<char>::from_str(input);
    let mut xmas_count = 0;

    for y in 0..m.height {
        for x in 0..m.width {
            if m.matrix[y][x] != TARGET_LETTERS[0] {
                continue;
            }

            for direction in Direction::all().iter() {
                m.set_position(x, y);

                for target_letter_index in 1..TARGET_LETTERS.len() {
                    let target_letter = TARGET_LETTERS[target_letter_index];

                    m.move_in_dir(direction);

                    if m.cur() != target_letter {
                        break;
                    }

                    if target_letter_index == TARGET_LETTERS.len() - 1 {
                        xmas_count += 1;
                    }
                }
            }
        }
    }

    return xmas_count.to_string();
}

fn part_2(input: &str) -> String {
    let mut m = TraversableMatrix::<char>::from_str(input);
    let mut x_mas_count = 0;

    for y in 0..m.height {
        for x in 0..m.width {
            if m.matrix[y][x] != 'A' {
                continue;
            }

            m.set_position(x, y);

            if ((m.peek_in_dir(&Direction::UpLeft) == Some('S')
                && m.peek_in_dir(&Direction::DownRight) == Some('M'))
                || (m.peek_in_dir(&Direction::UpLeft) == Some('M')
                    && m.peek_in_dir(&Direction::DownRight) == Some('S')))
                && ((m.peek_in_dir(&Direction::UpRight) == Some('S')
                    && m.peek_in_dir(&Direction::DownLeft) == Some('M'))
                    || (m.peek_in_dir(&Direction::UpRight) == Some('M')
                        && m.peek_in_dir(&Direction::DownLeft) == Some('S')))
            {
                x_mas_count += 1;
            }
        }
    }

    return x_mas_count.to_string();
}

fn main() {
    let input = read_input("2024", "04");

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
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
            "
        .trim();

        assert_eq!(part_1(input), "18");
    }

    #[test]
    fn test_part_2() {
        let input = r"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
        "
        .trim();

        assert_eq!(part_2(input), "9");
    }
}
