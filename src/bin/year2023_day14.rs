use std::time::Instant;

use aoc_rust::read_input;
use itertools::Itertools;

fn tilt_north(matrix: &mut Vec<Vec<char>>) {
    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if matrix[row][col] != '.' && matrix[row][col] != '#' {
                for target_row in (0..=row).rev() {
                    if target_row == 0 || matrix[target_row - 1][col] != '.' {
                        if target_row != row {
                            matrix[target_row][col] = matrix[row][col];
                            matrix[row][col] = '.';
                        }
                        break;
                    }
                }
            }
        }
    }
}

fn tilt_west(matrix: &mut Vec<Vec<char>>) {
    for col in 0..matrix[0].len() {
        for row in 0..matrix.len() {
            if matrix[row][col] != '.' && matrix[row][col] != '#' {
                for target_col in (0..=col).rev() {
                    if target_col == 0 || matrix[row][target_col - 1] != '.' {
                        if target_col != col {
                            matrix[row][target_col] = matrix[row][col];
                            matrix[row][col] = '.';
                        }
                        break;
                    }
                }
            }
        }
    }
}

fn tilt_south(matrix: &mut Vec<Vec<char>>) {
    for row in (0..matrix.len()).rev() {
        for col in 0..matrix[row].len() {
            if matrix[row][col] != '.' && matrix[row][col] != '#' {
                for target_row in row..matrix.len() {
                    if target_row == matrix.len() - 1 || matrix[target_row + 1][col] != '.' {
                        if target_row != row {
                            matrix[target_row][col] = matrix[row][col];
                            matrix[row][col] = '.';
                        }
                        break;
                    }
                }
            }
        }
    }
}

fn tilt_east(matrix: &mut Vec<Vec<char>>) {
    for col in (0..matrix[0].len()).rev() {
        for row in 0..matrix.len() {
            if matrix[row][col] != '.' && matrix[row][col] != '#' {
                for target_col in col..matrix[0].len() {
                    if target_col == matrix[0].len() - 1 || matrix[row][target_col + 1] != '.' {
                        if col != target_col {
                            matrix[row][target_col] = matrix[row][col];
                            matrix[row][col] = '.';
                        }
                        break;
                    }
                }
            }
        }
    }
}

fn cycle(matrix: &mut Vec<Vec<char>>) {
    tilt_north(matrix);
    tilt_west(matrix);
    tilt_south(matrix);
    tilt_east(matrix);
}

fn get_total_load(matrix: &Vec<Vec<char>>) -> usize {
    let mut total_load: usize = 0;

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if matrix[row][col] == 'O' {
                total_load += matrix.len() - row;
            }
        }
    }

    total_load
}

fn part_1(input: &str) -> String {
    let mut matrix = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    tilt_north(&mut matrix);
    get_total_load(&matrix).to_string()
}

fn equal(matrix_a: &Vec<Vec<char>>, matrix_b: &Vec<Vec<char>>) -> bool {
    for row in 0..matrix_a.len() {
        for col in 0..matrix_a[row].len() {
            if matrix_a[row][col] == 'O' && matrix_b[row][col] != 'O' {
                return false;
            }
        }
    }

    true
}

fn part_2(input: &str) -> String {
    let original_matrix = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut matrices: Vec<Vec<Vec<char>>> = vec![original_matrix];

    let mut cycles = 0;

    let mut loop_size = 0;

    while loop_size == 0 {
        cycles += 1;

        let mut matrix = matrices.last().unwrap().clone();
        cycle(&mut matrix);

        for i in (0..matrices.len()).rev() {
            if equal(&matrix, &matrices[i]) {
                loop_size = cycles - i;
            }
        }

        matrices.push(matrix);
    }

    let mut matrix = matrices.last_mut().unwrap();

    for _ in 0..((1000000000 - cycles) % loop_size) {
        cycle(&mut matrix);
    }

    get_total_load(&matrix).to_string()
}

fn main() {
    let input = read_input("2023", "14");

    let start_part_1 = Instant::now();
    let part_1_result = part_1(&input);
    let part_1_time = start_part_1.elapsed();

    println!();
    println!("Part 1: {} ({:?})", part_1_result, part_1_time);

    let start_part_2 = Instant::now();
    let part_2_result = part_2(&input);
    let part_2_time = start_part_2.elapsed();

    println!("Part 2: {} ({:?})", part_2_result, part_2_time);
    //println!("Part 2 Brute Force: {}", part_2_bruteforce(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
            .trim();

        assert_eq!(part_1(input), "136");
    }

    #[test]
    fn test_part_2() {
        let input = r"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
        "
        .trim();

        assert_eq!(part_2(input), "64");
    }
}
