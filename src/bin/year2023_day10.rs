use std::time::Instant;

use aoc_rust::read_input;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[derive(Clone, Copy)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

fn char_at(matrix: &Vec<Vec<char>>, (x, y): (isize, isize)) -> Option<char> {
    if x >= 0 && y >= 0 && (y as usize) < matrix.len() && (x as usize) < matrix[0].len() {
        Some(matrix[y as usize][x as usize])
    } else {
        None
    }
}

fn move_to((x, y): (isize, isize), direction: Direction) -> (isize, isize) {
    match direction {
        Direction::Top => (x, y - 1),
        Direction::Right => (x + 1, y),
        Direction::Bottom => (x, y + 1),
        Direction::Left => (x - 1, y),
    }
}

fn connections_at(matrix: &Vec<Vec<char>>, pos: (isize, isize)) -> Option<(Direction, Direction)> {
    match char_at(&matrix, pos) {
        Some('|') => Some((Direction::Top, Direction::Bottom)),
        Some('-') => Some((Direction::Left, Direction::Right)),
        Some('L') => Some((Direction::Top, Direction::Right)),
        Some('J') => Some((Direction::Top, Direction::Left)),
        Some('7') => Some((Direction::Bottom, Direction::Left)),
        Some('F') => Some((Direction::Bottom, Direction::Right)),
        _ => None,
    }
}

fn can_move_at(matrix: &Vec<Vec<char>>, pos: (isize, isize), direction: Direction) -> bool {
    match char_at(&matrix, move_to(pos, direction)) {
        Some('|') => match direction {
            Direction::Top | Direction::Bottom => true,
            _ => false,
        },
        Some('-') => match direction {
            Direction::Left | Direction::Right => true,
            _ => false,
        },
        Some('L') => match direction {
            Direction::Left | Direction::Bottom => true,
            _ => false,
        },
        Some('J') => match direction {
            Direction::Right | Direction::Bottom => true,
            _ => false,
        },
        Some('7') => match direction {
            Direction::Right | Direction::Top => true,
            _ => false,
        },
        Some('F') => match direction {
            Direction::Left | Direction::Top => true,
            _ => false,
        },
        _ => false,
    }
}

fn get_start(matrix: &Vec<Vec<char>>) -> (isize, isize) {
    for (y, _) in matrix.iter().enumerate() {
        for (x, _) in matrix[y].iter().enumerate() {
            let pos = (x as isize, y as isize);

            if char_at(&matrix, pos) == Some('S') {
                return pos;
            }
        }
    }

    (0, 0)
}

fn traverse(
    matrix: &Vec<Vec<char>>,
    current_pos: (isize, isize),
    prev_pos: (isize, isize),
    start_pos: (isize, isize),
    count: usize,
    mut path: Option<&mut Vec<(isize, isize)>>,
) -> Option<usize> {
    if let Some(p) = &mut path {
        p.push(current_pos);
    }

    if count > 0 && start_pos == current_pos {
        return Some(count / 2);
    }

    if let Some((dir_1, dir_2)) = connections_at(&matrix, current_pos) {
        if can_move_at(matrix, current_pos, dir_1) {
            let next_pos = move_to(current_pos, dir_1);

            if next_pos != prev_pos {
                return traverse(&matrix, next_pos, current_pos, start_pos, count + 1, path);
            }
        }

        if can_move_at(matrix, current_pos, dir_2) {
            let next_pos = move_to(current_pos, dir_2);

            if next_pos != prev_pos {
                return traverse(&matrix, next_pos, current_pos, start_pos, count + 1, path);
            } else {
                return None;
            }
        } else {
            return None;
        }
    } else {
        None
    }
}

fn part_1(input: &str) -> String {
    let mut matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let start_pos = get_start(&matrix);

    (get_loop_len(&mut matrix, start_pos) / 2).to_string()
}

fn insert_into_scaled_matrix(
    scaled_matrix: &mut Vec<Vec<u8>>,
    original_matrix: &Vec<Vec<char>>,
    original_matrix_pos: (isize, isize),
    marker: u8,
) {
    let x = original_matrix_pos.0 as usize * 3;
    let y = original_matrix_pos.1 as usize * 3;

    if scaled_matrix[y][x] != 9 {
        // already inserted something here
        return;
    }

    let mask = match char_at(original_matrix, original_matrix_pos) {
        Some('|') => 0b010010010,
        Some('-') => 0b000111000,
        Some('L') => 0b010011000,
        Some('J') => 0b010110000,
        Some('7') => 0b000110010,
        Some('F') => 0b000011010,
        Some('.') => 0b111111111,
        _ => panic!(),
    };

    scaled_matrix[y][x] = if mask & 0b100000000 == 0b100000000 {
        marker
    } else {
        0
    };

    scaled_matrix[y][x + 1] = if mask & 0b010000000 == 0b010000000 {
        marker
    } else {
        0
    };

    scaled_matrix[y][x + 2] = if mask & 0b001000000 == 0b001000000 {
        marker
    } else {
        0
    };

    scaled_matrix[y + 1][x] = if mask & 0b000100000 == 0b000100000 {
        marker
    } else {
        0
    };

    scaled_matrix[y + 1][x + 1] = if mask & 0b000010000 == 0b000010000 {
        marker
    } else {
        0
    };

    scaled_matrix[y + 1][x + 2] = if mask & 0b000001000 == 0b000001000 {
        marker
    } else {
        0
    };

    scaled_matrix[y + 2][x] = if mask & 0b000000100 == 0b0000000100 {
        marker
    } else {
        0
    };

    scaled_matrix[y + 2][x + 1] = if mask & 0b000000010 == 0b0000000010 {
        marker
    } else {
        0
    };

    scaled_matrix[y + 2][x + 2] = if mask & 0b000000001 == 0b0000000001 {
        marker
    } else {
        0
    };
}

fn print_matrix<T>(matrix: &Vec<Vec<T>>)
where
    T: ToString,
{
    println!(
        "SCALED -> \n{}",
        matrix
            .iter()
            .map(|row| row.iter().map(|n| n.to_string()).join(""))
            .join("\n")
    );
}

// recursively goes through the entire matrix, populating enclosedness by replacing 0s with:
// 3 - not enclosed in the loop
// 4 - enclosed in the loop
//
// returns whether pos is enclosed in the scaled_matrix's loop
fn populate_enclosed(scaled_matrix: &mut Vec<Vec<u8>>, (x, y): (isize, isize)) {
    if x < 0
        || y < 0
        || y as usize >= scaled_matrix.len()
        || x as usize >= scaled_matrix[y as usize].len()
        || (scaled_matrix[y as usize][x as usize] != 0
            && scaled_matrix[y as usize][x as usize] != 1)
    {
        return;
    }

    scaled_matrix[y as usize][x as usize] = 3;

    populate_enclosed(scaled_matrix, move_to((x, y), Direction::Right));
    populate_enclosed(scaled_matrix, move_to((x, y), Direction::Bottom));
    populate_enclosed(scaled_matrix, move_to((x, y), Direction::Left));
    populate_enclosed(scaled_matrix, move_to((x, y), Direction::Top));
}

fn get_loop_path(matrix: &mut Vec<Vec<char>>, start_pos: (isize, isize)) -> Vec<(isize, isize)> {
    for ch in ['|', '-', 'L', '7', 'J', 'F'] {
        let mut path: Vec<(isize, isize)> = vec![];
        matrix[start_pos.1 as usize][start_pos.0 as usize] = ch;

        if traverse(&matrix, start_pos, start_pos, start_pos, 0, Some(&mut path)).is_some() {
            return path;
        }
    }

    vec![]
}

fn get_loop_len(matrix: &mut Vec<Vec<char>>, start_pos: (isize, isize)) -> usize {
    for ch in ['|', '-', 'L', '7', 'J', 'F'] {
        matrix[start_pos.1 as usize][start_pos.0 as usize] = ch;

        if let Some(count) = traverse(&matrix, start_pos, start_pos, start_pos, 0, None) {
            println!("Hello {ch}");
            return count;
        }
    }

    0
}

fn part_2(input: &str) -> String {
    let start_part_2 = Instant::now();
    let mut matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let start_pos = get_start(&matrix);

    let path = get_loop_path(&mut matrix, start_pos);

    println!("1 - {:?}", start_part_2.elapsed());

    // create a scaled matrix with 3x resolution
    // this allows us to handle the "fit through pipes" thingy..
    // basically we map each tile to a 3x3 tile which represents each shape
    // e.g. | is represented as:
    // 0 1 0
    // 0 1 0
    // 0 1 0
    //
    // and L is represented as:
    //
    // 0 1 0
    // 0 1 1
    // 0 0 0
    //
    //
    //
    // begin by simply creating a 3xwidth 3xheight matrix filled with 9s (sentinel value indicating
    // "unknown")
    let s2 = Instant::now();
    let mut scaled_matrix: Vec<Vec<u8>> = vec![vec![9; matrix[0].len() * 3]; matrix.len() * 3];

    // now go through the loop path and insert tiles into the scaled matrix
    // they'll have the value "2" as they're part of the loop. More on this later...

    for &pos in &path {
        insert_into_scaled_matrix(&mut scaled_matrix, &matrix, pos, 2);
    }

    // now go through the original matrix and fill out the scaled_matrix
    // . will be filled as 0
    // pipes will be filled as 1
    // if a pipe was a part of the loop, it will remain a "2"
    // due to the fact that insert_into_scaled_matrix does not overwrite
    // and we already inserted the 2s.

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            insert_into_scaled_matrix(
                &mut scaled_matrix,
                &matrix,
                (x as isize, y as isize),
                if matrix[y][x] == '.' { 0 } else { 1 },
            );
        }
    }
    println!("Build scaled matrix: {:?}", s2.elapsed());

    // now we go through the scaled matrix and mark nodes that
    // are surrounded by "inside" nodes at all sides as "inside"
    // and as "outside" otherwise
    // where "inside" = 4 and "outside" = 3

    let s3 = Instant::now();
    populate_enclosed(&mut scaled_matrix, (0, 0));
    println!("Populate enclosed: {:?}", s3.elapsed());

    let mut enclosed_count = 0;

    // finally, iterate through the scaled_matrix, looking for 3x3 squares filled with "4"s
    // count each square as an enclosed tile in the original matrix..
    let s4 = Instant::now();
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            // if the center node is "4", that must mean the entire tile is filled with "4"s
            let center = scaled_matrix[y * 3 + 1][x * 3 + 1];

            if center == 0 || center == 1 {
                enclosed_count += 1;
            }
        }
    }

    println!("Count enclosed: {:?}", s4.elapsed());

    //print_matrix(&matrix);
    //print_matrix(&scaled_matrix);

    enclosed_count.to_string()
}

fn main() {
    let input = read_input("2023", "10");

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
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
        "
        .trim();

        assert_eq!(part_1(input), "8");
    }

    #[test]
    fn test_part_2_1() {
        let input = r"
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
        "
        .trim();

        assert_eq!(part_2(input), "4");
    }

    #[test]
    fn test_part_2_2() {
        let input = r"
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
        "
        .trim();

        assert_eq!(part_2(input), "8");
    }
}
