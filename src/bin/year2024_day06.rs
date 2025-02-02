use std::{panic, time::Instant};

use aoc_rust::{
    direction::Direction, read_input, traversable_matrix::TraversableMatrix, uvec2::UVec2,
};
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSlice,
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

#[derive(Clone, Copy, Debug)]
struct Obstacle {
    position: UVec2,
    // bit is raised for each direction that the guard moved in before colliding with the obstacle
    collision_directions: u8,
}

struct MatrixObstacles {
    // index is determined by matrix coords (y * width + x)
    all: Vec<Obstacle>,
    by_y: Vec<Vec<usize>>,
    by_x: Vec<Vec<usize>>,
    matrix_width: usize,
    visited_indices: Vec<usize>,
}

impl MatrixObstacles {
    fn new(m: &TraversableMatrix<char>) -> MatrixObstacles {
        let mut obstacles = MatrixObstacles {
            all: vec![
                Obstacle {
                    position: UVec2 { x: 0, y: 0 },
                    collision_directions: 0
                };
                m.width * m.height
            ],
            visited_indices: vec![],
            by_y: vec![vec![]; m.height],
            by_x: vec![vec![]; m.width],
            matrix_width: m.width,
        };

        for y in 0..m.height {
            for x in 0..m.width {
                if m.matrix[y][x] == '#' {
                    MatrixObstacles::insert_obstacle(&mut obstacles, x, y);
                }
            }
        }

        return obstacles;
    }

    fn get_1d_index(&self, x: usize, y: usize) -> usize {
        y * self.matrix_width + x
    }

    // returns the index of the obstacle reached by going in the specified direction from the
    // specified position
    fn find_obstacle(&mut self, pos: UVec2, direction: Direction) -> Option<usize> {
        match direction {
            Direction::Up => {
                for &y in self.by_x[pos.x].iter().rev() {
                    if y < pos.y {
                        return Some(self.get_1d_index(pos.x, y));
                    }
                }

                None
            }

            Direction::Right => {
                for &x in self.by_y[pos.y].iter() {
                    if x > pos.x {
                        return Some(self.get_1d_index(x, pos.y));
                    }
                }

                None
            }

            Direction::Down => {
                for &y in self.by_x[pos.x].iter() {
                    if y > pos.y {
                        return Some(self.get_1d_index(pos.x, y));
                    }
                }

                None
            }

            Direction::Left => {
                for &x in self.by_y[pos.y].iter().rev() {
                    if x < pos.x {
                        return Some(self.get_1d_index(x, pos.y));
                    }
                }

                None
            }

            _ => None,
        }
    }

    // returns true if navigating from specified starting position and starting direction
    // leads to a loop
    //
    // navigates only using obstacles
    // loop is found if an obstacle is approached from the same direction twice
    // loop is not found if we exit the matrix while navigating
    fn has_loop(&mut self, starting_position: UVec2, starting_direction: Direction) -> bool {
        let mut pos = starting_position;
        let mut direction = starting_direction;

        while let Some(index) = self.find_obstacle(pos, direction) {
            let bitmask = 2 << direction as u8;
            let obstacle = &mut self.all[index];

            //println!("collided while moving {:?} with {:?}", direction, obstacle);

            // if obstacle was already collided from this direction, we have a loop
            if obstacle.collision_directions & bitmask > 0 {
                return true;
            }

            // remember that we collided with this obstacle from this direction
            obstacle.collision_directions |= bitmask;
            self.visited_indices.push(index);

            // we set the position of the guard in relation to the collided obstacle
            // and the direction the guard was moving in before colliding with the obstacle
            // if we collided an obstacle at pos (3, 2) and we were moving up,
            // then we are at (3, 3) (below the obstacle)
            //
            pos.x = match direction {
                Direction::Right => obstacle.position.x - 1,
                Direction::Left => obstacle.position.x + 1,
                _ => pos.x,
            };
            pos.y = match direction {
                Direction::Up => obstacle.position.y + 1,
                Direction::Down => obstacle.position.y - 1,
                _ => pos.y,
            };
            direction = get_rotated_direction(direction);

            //println!("searching from pos {:?} and dir {:?}", pos, direction);
        }

        //println!("XXXXXX no loop found");

        return false;
    }

    fn insert_obstacle(&mut self, x: usize, y: usize) {
        let index = self.get_1d_index(x, y);

        self.all[index].position.x = x;
        self.all[index].position.y = y;
        self.by_y[y].push(x);
        self.by_y[y].sort();
        self.by_x[x].push(y);
        self.by_x[x].sort();
    }

    fn remove_obstacle(&mut self, x: usize, y: usize) {
        let index = self.get_1d_index(x, y);

        self.all[index].position.x = 0;
        self.all[index].position.y = 0;
        self.all[index].collision_directions = 0;

        if let Some(i) = self.by_y[y].iter().position(|&ox| ox == x) {
            self.by_y[y].remove(i);
        }

        if let Some(i) = self.by_x[x].iter().position(|&oy| oy == y) {
            self.by_x[x].remove(i);
        }
    }

    fn reset_collisions(&mut self) {
        for &i in self.visited_indices.iter() {
            self.all[i].collision_directions = 0;
        }

        self.visited_indices.clear();
    }
}

fn part_2(input: &str) -> String {
    let mut m = TraversableMatrix::<char>::from_str(input);
    let starting_position = get_starting_position(&mut m);
    let distinct_positions = get_distinct_visited_positions(&mut m);

    distinct_positions
        .par_chunks(distinct_positions.len() / 20)
        .map(|positions| {
            //println!("chunk size: {}", positions.len());
            let mut obstacles = MatrixObstacles::new(&m);
            let mut loops = 0;
            //println!("[START] inserting obstacle at {:?}", pos);
            for &pos in positions {
                obstacles.insert_obstacle(pos.x, pos.y);

                if obstacles.has_loop(starting_position, Direction::Up) {
                    loops += 1
                }

                obstacles.remove_obstacle(pos.x, pos.y);
                obstacles.reset_collisions();
            }

            return loops;
        })
        .sum::<usize>()
        .to_string()
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
