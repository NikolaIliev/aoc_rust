use std::time::Instant;

use aoc_rust::read_input;
use itertools::Itertools;

fn is_mirrored(slice: &[usize]) -> bool {
    if slice.len() % 2 == 1 {
        return false;
    }

    for i in 0..slice.len() / 2 {
        if slice[i] != slice[slice.len() - 1 - i] {
            return false;
        }
    }

    return true;
}

fn get_reflection_lines(scores: &Vec<usize>) -> Vec<usize> {
    let mut lines: Vec<usize> = vec![];

    // first look on the left side
    for i in (1..scores.len()).rev() {
        if scores[0] == scores[i] {
            let slice = &scores[0..=i];

            if is_mirrored(slice) {
                //println!(
                //"found left reflection at idx: {i} -> {:?}; entire: {:?}; SCORE: {}",
                //&scores[0..=i],
                //scores,
                //slice.len() / 2
                //);
                lines.push(i / 2);
            }
        }
    }

    // then on the right side

    for i in 0..scores.len() - 1 {
        if scores[scores.len() - 1] == scores[i] {
            let slice = &scores[i..scores.len()];

            if is_mirrored(slice) {
                //println!(
                //"found right reflection at idx: {i} -> {:?}; entire: {:?}; SCORE: {}",
                //&scores[i..scores.len()],
                //scores,
                //slice.len() / 2 + i
                //);

                lines.push(scores.len() / 2 + i / 2);
            }
        }
    }

    lines
}

fn invert_char_at(matrix: &mut Vec<Vec<char>>, row: usize, col: usize) {
    matrix[row][col] = if matrix[row][col] == '#' { '.' } else { '#' };
}

fn get_scores(
    mut matrix: &mut Vec<Vec<char>>,
    expand_smudges: bool,
) -> Vec<(Vec<usize>, Vec<usize>)> {
    let row_scores = matrix
        .iter()
        .map(|row| {
            row.iter().enumerate().fold(0, |sum, (idx, &ch)| {
                sum + if ch == '#' {
                    (10 as usize).pow(idx as u32)
                } else {
                    0
                }
            })
        })
        .collect_vec();

    let mut col_scores: Vec<usize> = vec![];

    for col in 0..matrix[0].len() {
        let mut sum = 0;
        for row in 0..matrix.len() {
            sum += if matrix[row][col] == '#' {
                (10 as usize).pow(row as u32)
            } else {
                0
            }
        }

        col_scores.push(sum);
    }

    let mut scores: Vec<(Vec<usize>, Vec<usize>)> = vec![];

    if expand_smudges {
        for i in 0..row_scores.len() {
            for j in i..row_scores.len() {
                let smudge_col_f = (row_scores[i].abs_diff(row_scores[j]) as f64).log10();

                if smudge_col_f % 1.0 == 0.0 {
                    let smudge_col = smudge_col_f as usize;

                    //println!("Found col smudge: ({i}, {smudge_col})");
                    //println!("Found col smudge: ({j}, {smudge_col})");

                    invert_char_at(&mut matrix, i, smudge_col);
                    scores.append(&mut get_scores(matrix, false));
                    invert_char_at(&mut matrix, i, smudge_col);
                }
            }
        }

        for i in 0..col_scores.len() {
            for j in i..col_scores.len() {
                let smudge_row_f = (col_scores[i].abs_diff(col_scores[j]) as f64).log10();

                if smudge_row_f % 1.0 == 0.0 {
                    let smudge_row = smudge_row_f as usize;

                    //println!(
                    //"Found row smudge: {i}, {j}, {smudge_row}, {}, {}, {}",
                    //col_scores[i],
                    //col_scores[j],
                    //col_scores[i].abs_diff(col_scores[j])
                    //);

                    invert_char_at(&mut matrix, smudge_row, i);
                    scores.append(&mut get_scores(matrix, false));
                    invert_char_at(&mut matrix, smudge_row, i);
                    invert_char_at(&mut matrix, smudge_row, j);
                    scores.append(&mut get_scores(matrix, false));
                    invert_char_at(&mut matrix, smudge_row, j);
                }
            }
        }
    } else {
        scores.push((row_scores, col_scores));
    }

    scores
}

fn parse_pattern(pattern: &str, expand_smudges: bool) -> Vec<(Vec<usize>, Vec<usize>)> {
    let mut matrix: Vec<Vec<char>> = pattern
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    get_scores(&mut matrix, expand_smudges)
}

fn part_1(input: &str) -> String {
    input
        .split("\n\n")
        .map(|pattern| {
            //println!("pattern:\n{pattern}");
            let scores = parse_pattern(pattern, false);

            let row_reflection_lines = get_reflection_lines(&scores[0].0);

            if row_reflection_lines.len() > 0 {
                //println!("Row reflection line: {}", row_reflection_lines[0]);
                return (row_reflection_lines[0] + 1) * 100;
            }

            let col_reflection_lines = get_reflection_lines(&scores[0].1);

            //println!("Col reflection line: {}", col_reflection_lines[0]);

            return col_reflection_lines[0] + 1;
        })
        .sum::<usize>()
        .to_string()
}

fn part_2(input: &str) -> String {
    input
        .split("\n\n")
        .map(|pattern| {
            //println!("pattern:\n{pattern}");
            let original_scores = parse_pattern(pattern, false);
            let original_row_reflection_lines = get_reflection_lines(&original_scores[0].0);
            let original_col_reflection_lines = get_reflection_lines(&original_scores[0].1);

            //println!(
            //"original_row_reflection_lines:\n{:?}",
            //original_row_reflection_lines
            //);
            //println!(
            //"original_col_reflection_lines:\n{:?}",
            //original_col_reflection_lines
            //);

            let smudge_scores = parse_pattern(pattern, true);

            smudge_scores
                .iter()
                .find_map(|(row_scores, col_scores)| {
                    //println!("=======");
                    let row_reflection_lines = get_reflection_lines(row_scores);
                    let col_reflection_lines = get_reflection_lines(col_scores);

                    //println!("row_reflection_lines:\n{:?}", row_reflection_lines);
                    //println!("col_reflection_lines:\n{:?}", col_reflection_lines);

                    if row_reflection_lines.len() > 0 {
                        if original_row_reflection_lines.len() == 0 {
                            //println!(">> new row reflection line: {}", row_reflection_lines[0]);
                            return Some((row_reflection_lines[0] + 1) * 100);
                        }

                        let new_row_reflection_line = row_reflection_lines
                            .iter()
                            .find(|&line| *line != original_row_reflection_lines[0]);

                        if new_row_reflection_line.is_some() {
                            //println!(
                            //">> new row reflection line: {}",
                            //*new_row_reflection_line.unwrap()
                            //);
                            return Some((*new_row_reflection_line.unwrap() + 1) * 100);
                        }
                    }

                    if col_reflection_lines.len() > 0 {
                        if original_col_reflection_lines.len() == 0 {
                            //println!(">> new col reflection line: {}", col_reflection_lines[0]);
                            return Some(col_reflection_lines[0] + 1);
                        }

                        let new_col_reflection_line = col_reflection_lines
                            .iter()
                            .find(|&line| *line != original_col_reflection_lines[0]);

                        if new_col_reflection_line.is_some() {
                            //println!(
                            //">> new col reflection line: {}",
                            //new_col_reflection_line.unwrap()
                            //);
                            return Some(*new_col_reflection_line.unwrap() + 1);
                        }
                    }

                    None
                })
                .unwrap()
        })
        .sum::<usize>()
        .to_string()
}

fn main() {
    let input = read_input("2023", "13");

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
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
        "
        .trim();

        assert_eq!(part_1(input), "405");
    }

    #[test]
    fn test_part_2() {
        let input = r"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
        "
        .trim();

        assert_eq!(part_2(input), "400");
    }
}
