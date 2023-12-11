use std::{collections::HashSet, time::Instant};

use aoc_rust::read_input;

fn get_galaxies(input: &str, expansion_factor: usize) -> Vec<(usize, usize)> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();
    let mut galaxies =
        input
            .lines()
            .enumerate()
            .fold(Vec::<(usize, usize)>::new(), |mut vec, (y, line)| {
                for (x, ch) in line.chars().enumerate() {
                    if ch == '#' {
                        vec.push((x, y));
                    }
                }

                vec
            });
    let mut galaxy_xs: HashSet<usize> = HashSet::new();
    let mut galaxy_ys: HashSet<usize> = HashSet::new();

    for (x, y) in &galaxies {
        galaxy_xs.insert(*x);
        galaxy_ys.insert(*y);
    }

    let x_gaps: Vec<usize> = (0..width)
        .fold(
            (Vec::<usize>::new(), 0),
            |(mut gaps_vec, mut gaps_count), x| {
                if !galaxy_xs.contains(&x) {
                    gaps_count += 1;
                }

                gaps_vec.push(gaps_count);
                (gaps_vec, gaps_count)
            },
        )
        .0;
    let y_gaps: Vec<usize> = (0..height)
        .fold(
            (Vec::<usize>::new(), 0),
            |(mut gaps_vec, mut gaps_count), y| {
                if !galaxy_ys.contains(&y) {
                    gaps_count += 1;
                }

                gaps_vec.push(gaps_count);
                (gaps_vec, gaps_count)
            },
        )
        .0;

    for galaxy_coords in &mut galaxies {
        galaxy_coords.0 += x_gaps[galaxy_coords.0] * (expansion_factor - 1);
        galaxy_coords.1 += y_gaps[galaxy_coords.1] * (expansion_factor - 1);
    }

    galaxies
}

fn get_sum_of_distances(galaxies: &Vec<(usize, usize)>) -> usize {
    let mut sum = 0;

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let (x_a, y_a) = galaxies[i];
            let (x_b, y_b) = galaxies[j];

            sum += x_a.abs_diff(x_b) + y_a.abs_diff(y_b);
        }
    }

    sum
}

fn part_1(input: &str) -> String {
    get_sum_of_distances(&get_galaxies(input, 2)).to_string()
}

fn part_2(input: &str) -> String {
    get_sum_of_distances(&get_galaxies(input, 1_000_000)).to_string()
}

fn main() {
    let input = read_input("2023", "11");

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
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
        "
        .trim();

        assert_eq!(part_1(input), "374");
    }

    #[test]
    fn test_part_2() {
        let input = r"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
        "
        .trim();

        assert_eq!(part_2(input), "8410");
    }
}
