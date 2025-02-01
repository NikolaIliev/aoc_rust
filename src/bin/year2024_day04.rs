use std::time::Instant;

use aoc_rust::{direction::Direction, read_input, traversable_matrix::TraversableMatrix};

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
