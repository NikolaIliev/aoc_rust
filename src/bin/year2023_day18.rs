use std::time::Instant;

use aoc_rust::read_input;
use itertools::Itertools;

#[derive(Debug)]
struct Line {
    pos: (isize, isize),
    length: isize,
    direction: (i8, i8),
}

fn parse_lines(input: &str, hex: bool) -> (Vec<Line>, isize) {
    let mut max_y = 0;

    let lines = input
        .lines()
        .fold(((0, 0), Vec::<Line>::new()), |(pos, mut vec), line| {
            let (direction_str, length_str, hex_str) =
                line.split_whitespace().collect_tuple().unwrap();

            let length = if hex {
                isize::from_str_radix(&hex_str[2..7], 16).unwrap()
            } else {
                length_str.parse::<isize>().unwrap()
            };

            let direction: (i8, i8) = if hex {
                match hex_str.chars().nth(7).unwrap() {
                    '0' => (1, 0),
                    '1' => (0, 1),
                    '2' => (-1, 0),
                    '3' => (0, -1),
                    _ => unreachable!(),
                }
            } else {
                match direction_str {
                    "R" => (1, 0),
                    "D" => (0, 1),
                    "L" => (-1, 0),
                    "U" => (0, -1),
                    _ => unreachable!(),
                }
            };

            let new_pos = (
                pos.0 + length as isize * direction.0 as isize,
                pos.1 + length as isize * direction.1 as isize,
            );

            max_y = max_y.max(new_pos.1);

            vec.push(Line {
                pos,
                length,
                direction,
            });

            (new_pos, vec)
        })
        .1;

    (lines, max_y)
}

fn surface_area(lines: &Vec<Line>, max_y: isize) -> isize {
    let mut surface_area = 0;

    // https://www.mathsisfun.com/geometry/area-irregular-polygons.html
    for line in lines {
        let area = match line.direction {
            (1, 0) => (max_y - line.pos.1 + 1) * line.length,
            (-1, 0) => -(max_y - line.pos.1 + 1) * line.length,
            (0, 1) | (0, -1) => 0,
            _ => unreachable!(),
        };

        surface_area += area;
    }

    // as our points represent the centre of 1x1x1 cubes we need to add additional surface area that we
    // didn't calculate above
    //
    // this is done by using this formula that I couldn't figure out myself so I copied from reddit
    // Annoying tbh
    surface_area += lines.into_iter().map(|line| line.length).sum::<isize>() / 2 + 1;

    surface_area
}

fn part_1(input: &str) -> String {
    let (lines, max_y) = parse_lines(input, false);

    surface_area(&lines, max_y).to_string()
}

fn part_2(input: &str) -> String {
    let (lines, max_y) = parse_lines(input, true);

    surface_area(&lines, max_y).to_string()
}

fn main() {
    let input = read_input("2023", "18");

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

    #[ignore]
    #[test]
    fn test_part_1() {
        let input = r"
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
        "
        .trim();

        assert_eq!(part_1(input), "");
    }

    #[ignore]
    #[test]
    fn test_part_2() {
        let input = r"".trim();

        assert_eq!(part_2(input), "");
    }
}
