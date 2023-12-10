use std::time::Instant;

use aoc_rust::read_input;

fn parse_input(input: &str) -> (Vec<Vec<u8>>, (usize, usize)) {
    let mut start_pos: (usize, usize) = (0, 0);
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();
    let mut matrix: Vec<Vec<u8>> = vec![vec![0; width * 3]; height * 3];

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start_pos = ((x * 3), (y * 3));
                continue;
            }

            if ch == '.' {
                continue;
            }

            let mask = match ch {
                '|' => 0b010010010,
                '-' => 0b000111000,
                'L' => 0b010011000,
                'J' => 0b010110000,
                '7' => 0b000110010,
                'F' => 0b000011010,
                _ => panic!(),
            };

            if mask & 0b100000000 == 0b100000000 {
                matrix[y * 3][x * 3] = 1;
            }

            if mask & 0b010000000 == 0b010000000 {
                matrix[y * 3][x * 3 + 1] = 1
            }

            if mask & 0b001000000 == 0b001000000 {
                matrix[y * 3][x * 3 + 2] = 1
            }

            if mask & 0b000100000 == 0b000100000 {
                matrix[y * 3 + 1][x * 3] = 1
            }

            if mask & 0b000010000 == 0b000010000 {
                matrix[y * 3 + 1][x * 3 + 1] = 1
            }

            if mask & 0b000001000 == 0b000001000 {
                matrix[y * 3 + 1][x * 3 + 2] = 1
            }

            if mask & 0b000000100 == 0b0000000100 {
                matrix[y * 3 + 2][x * 3] = 1
            }

            if mask & 0b000000010 == 0b0000000010 {
                matrix[y * 3 + 2][x * 3 + 1] = 1
            }

            if mask & 0b000000001 == 0b0000000001 {
                matrix[y * 3 + 2][x * 3 + 2] = 1
            }
        }
    }

    // fill in S
    matrix[start_pos.1 + 1][start_pos.0 + 1] = 1;

    if matrix[start_pos.1 + 1][start_pos.0 - 1] == 1 {
        matrix[start_pos.1 + 1][start_pos.0] = 1;
    }

    if matrix[start_pos.1 + 1][start_pos.0 + 3] == 1 {
        matrix[start_pos.1 + 1][start_pos.0 + 2] = 1;
    }

    if matrix[start_pos.1 - 1][start_pos.0 + 1] == 1 {
        matrix[start_pos.1][start_pos.0 + 1] = 1;
    }

    if matrix[start_pos.1 + 3][start_pos.0 + 1] == 1 {
        matrix[start_pos.1 + 2][start_pos.0 + 1] = 1;
    }

    (matrix, start_pos)
}

fn find_loop(
    matrix: &Vec<Vec<u8>>,
    current_pos: (usize, usize),
    prev_pos: (usize, usize),
    start_pos: (usize, usize),
    path: &mut Vec<(usize, usize)>,
) {
    if !path.is_empty() && current_pos == start_pos {
        return;
    }

    path.push(current_pos);

    let (x, y) = current_pos;

    if matrix[y][x + 1] == 1 {
        let new_pos = (x + 1, y);

        if new_pos != prev_pos {
            return find_loop(matrix, new_pos, current_pos, start_pos, path);
        }
    }

    if matrix[y][x - 1] == 1 {
        let new_pos = (x - 1, y);

        if new_pos != prev_pos {
            return find_loop(matrix, new_pos, current_pos, start_pos, path);
        }
    }

    if matrix[y + 1][x] == 1 {
        let new_pos = (x, y + 1);

        if new_pos != prev_pos {
            return find_loop(matrix, new_pos, current_pos, start_pos, path);
        }
    }

    if matrix[y - 1][x] == 1 {
        let new_pos = (x, y - 1);

        if new_pos != prev_pos {
            return find_loop(matrix, new_pos, current_pos, start_pos, path);
        }
    }
}

fn part_1(input: &str) -> String {
    let (matrix, start_pos) = parse_input(input);
    let mut loop_path: Vec<(usize, usize)> = vec![];

    let start_center_pos = (start_pos.0 + 1, start_pos.1 + 1);

    find_loop(
        &matrix,
        start_center_pos,
        start_center_pos,
        start_center_pos,
        &mut loop_path,
    );

    (loop_path.len() / 6).to_string()
}

fn fill(scaled_matrix: &mut Vec<Vec<u8>>, pos: (usize, usize), fill_with: u8, avoid: u8) {
    let (x, y) = pos;
    scaled_matrix[y as usize][x as usize] = fill_with;

    if x < scaled_matrix[y].len() - 1
        && scaled_matrix[y][(x) + 1] != avoid
        && scaled_matrix[y][(x) + 1] != fill_with
    {
        fill(scaled_matrix, (x + 1, y), fill_with, avoid);
    }

    if (y) < scaled_matrix.len() - 1
        && scaled_matrix[(y) + 1][x] != avoid
        && scaled_matrix[(y) + 1][x] != fill_with
    {
        fill(scaled_matrix, (x, y + 1), fill_with, avoid);
    }

    if x > 0 && scaled_matrix[y][(x) - 1] != avoid && scaled_matrix[y][(x) - 1] != fill_with {
        fill(scaled_matrix, (x - 1, y), fill_with, avoid);
    }

    if y > 0 && scaled_matrix[(y) - 1][x] != avoid && scaled_matrix[(y) - 1][x] != fill_with {
        fill(scaled_matrix, (x, y - 1), fill_with, avoid);
    }
}

fn part_2(input: &str) -> String {
    let (mut matrix, start_pos) = parse_input(input);
    let mut loop_path: Vec<(usize, usize)> = vec![];

    let start_center_pos = (start_pos.0 + 1, start_pos.1 + 1);

    find_loop(
        &matrix,
        start_center_pos,
        start_center_pos,
        start_center_pos,
        &mut loop_path,
    );

    let loop_marker = 2;
    let enclosed_marker = 3;

    for (x, y) in loop_path {
        matrix[y][x] = loop_marker;
    }

    // on the row we're certain there's a part of the loop (starting point row)
    // -> we're looking for a cell which comes after a part of the loop
    // we're certain that this cell is INSIDE the loop, because we're starting from the edge
    // note: if the starting point is actually the corner of the loop,
    // and there's no part of the loop to the left or right of it, this won't work
    // but i've not seen such inputs tbh :D
    let inside_loop_pos = (
        matrix[start_center_pos.1]
            .iter()
            .enumerate()
            .find_map(|(x, &n)| {
                if x < matrix[0].len() - 1 && n == 2 && matrix[start_center_pos.1][x + 1] != 2 {
                    Some(x + 1)
                } else {
                    None
                }
            })
            .unwrap(),
        start_center_pos.1,
    );

    fill(&mut matrix, inside_loop_pos, enclosed_marker, loop_marker);

    let mut enclosed_count = 0;

    for y in 0..matrix.len() / 3 {
        for x in 0..matrix[y].len() / 3 {
            let center = matrix[y * 3 + 1][x * 3 + 1];

            if center == enclosed_marker {
                enclosed_count += 1;
            }
        }
    }

    return enclosed_count.to_string();
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
