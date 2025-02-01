use itertools::Itertools;
use std::fmt::Debug;
use std::str::FromStr;

use crate::{direction::Direction, uvec2::UVec2};

#[derive(Debug)]
pub struct TraversableMatrix<T>
where
    T: FromStr,
{
    pub matrix: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
    pub position: UVec2,
}

impl<T> TraversableMatrix<T>
where
    T: FromStr + Copy,
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

    pub fn peek_in_dir(&self, direction: &Direction) -> Option<T> {
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

    pub fn move_in_dir(&mut self, direction: &Direction) {
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
