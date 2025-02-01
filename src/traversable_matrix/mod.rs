use itertools::Itertools;
use std::fmt::{Debug, Display};
use std::str::FromStr;

use crate::{direction::Direction, uvec2::UVec2};

#[derive(Debug)]
pub struct TraversableMatrix<T>
where
    T: FromStr + Display,
{
    pub matrix: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
    pub position: UVec2,
}

impl<T> TraversableMatrix<T>
where
    T: FromStr + Display + Copy,
    <T as FromStr>::Err: Debug,
{
    pub fn from_str(s: &str) -> TraversableMatrix<T> {
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

    pub fn cur(&self) -> T {
        return self.matrix[self.position.y][self.position.x];
    }

    pub fn set_position(&mut self, x: usize, y: usize) {
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

    pub fn pos_in_dir(&self, direction: Direction) -> Option<UVec2> {
        let UVec2 { x, y } = self.position;

        match direction {
            Direction::Up => {
                if self.can_move_up() {
                    Some(UVec2 { x, y: y - 1 })
                } else {
                    None
                }
            }
            Direction::Right => {
                if self.can_move_right() {
                    Some(UVec2 { x: x + 1, y })
                } else {
                    None
                }
            }
            Direction::Down => {
                if self.can_move_down() {
                    Some(UVec2 { x, y: y + 1 })
                } else {
                    None
                }
            }
            Direction::Left => {
                if self.can_move_left() {
                    Some(UVec2 { x: x - 1, y })
                } else {
                    None
                }
            }
            Direction::UpLeft => {
                if self.can_move_up() && self.can_move_left() {
                    Some(UVec2 { x: x - 1, y: y - 1 })
                } else {
                    None
                }
            }
            Direction::UpRight => {
                if self.can_move_up() && self.can_move_right() {
                    Some(UVec2 { x: x + 1, y: y - 1 })
                } else {
                    None
                }
            }
            Direction::DownLeft => {
                if self.can_move_down() && self.can_move_left() {
                    Some(UVec2 { x: x - 1, y: y + 1 })
                } else {
                    None
                }
            }
            Direction::DownRight => {
                if self.can_move_down() && self.can_move_right() {
                    Some(UVec2 { x: x + 1, y: y + 1 })
                } else {
                    None
                }
            }
        }
    }

    pub fn peek_in_dir(&self, direction: Direction) -> Option<T> {
        let pos = self.pos_in_dir(direction);

        match pos {
            Some(pos) => Some(self.matrix[pos.y][pos.x]),
            None => None,
        }
    }

    pub fn move_in_dir(&mut self, direction: Direction) {
        if let Some(pos) = self.pos_in_dir(direction) {
            self.position.x = pos.x;
            self.position.y = pos.y
        }
    }

    pub fn print(&self) {
        println!(
            "{}",
            self.matrix
                .iter()
                .map(|row| row.into_iter().join(""))
                .join("\n")
        );
    }
}
