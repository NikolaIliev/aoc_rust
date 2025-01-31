use std::{f64::INFINITY, process::Command, time::Instant};

use aoc_rust::read_input;
use itertools::Itertools;

#[derive(Debug)]
struct Hailstone {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
}

fn parse_hailstones(input: &str) -> Vec<Hailstone> {
    input
        .lines()
        .map(|line| {
            let (vals, speeds) = line.split_once(" @ ").unwrap();
            let (x, y) = vals
                .split(", ")
                .map(|s| s.trim().parse::<f64>().unwrap())
                .collect_tuple()
                .unwrap();

            let (dx, dy) = speeds
                .split(", ")
                .map(|s| s.trim().parse::<f64>().unwrap())
                .collect_tuple()
                .unwrap();

            Hailstone { x, y, dx, dy }
        })
        .collect_vec()
}

fn gcd(mut a: f64, mut b: f64) -> f64 {
    while b > 0.0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }

    a
}

fn lcm(a: f64, b: f64) -> f64 {
    a * b / gcd(a, b)
}

fn part_1(input: &str, intersection_min: f64, intersection_max: f64) -> String {
    let hailstones = parse_hailstones(input);
    let mut intersections = 0;

    for i in 0..hailstones.len() - 1 {
        let ha = &hailstones[i];
        let lcm_t = lcm(ha.dx, ha.dy);

        for j in i + 1..hailstones.len() {
            let hb = &hailstones[j];

            // ha.x + t * ha.dx = hb.x + t * hb.dx
            // ha.y + s * ha.dy = hb.y + s * ha.dy
            //
            // or:
            //
            // Hailstone A:
            // x = ha.x + t * ha.dx
            // y = ha.y + t * ha.dy
            //
            // Hailstone B:
            // x = hb.x + s * hb.dx
            // y = hb.y + s * hb.dy
            //
            // Solve for same (x, y):
            //
            // ha.x + t * ha.dx = hb.x + s * hb.dx
            // ha.y + t * ha.dy = hb.y + s * hb.dy
            //
            // 1. Find LCM(ha.dx, ha.dy) -> multiply each equation to eliminate t
            // 2. Find LCM(hb.dx, hb.dy) -> multiply each equation to eliminate s
            //
            // Solve for t and s
            // where t & s are points in time which may differ
            let s = (ha.x * (lcm_t / ha.dx) - ha.y * (lcm_t / ha.dy) - hb.x * (lcm_t / ha.dx)
                + hb.y * (lcm_t / ha.dy))
                / (hb.dx * (lcm_t / ha.dx) - hb.dy * (lcm_t / ha.dy));
            let lcm_s = lcm(hb.dx, hb.dy);
            let t = (hb.x * (lcm_s / hb.dx) - ha.x * (lcm_s / hb.dx) + ha.y * (lcm_s / hb.dy)
                - hb.y * (lcm_s / hb.dy))
                / (ha.dx * (lcm_s / hb.dx) - ha.dy * (lcm_s / hb.dy));

            if t > 0.0 && s > 0.0 && t != INFINITY && s != INFINITY {
                let x = ha.x + t * ha.dx;
                let y = ha.y + t * ha.dy;

                if x >= intersection_min
                    && x <= intersection_max
                    && y >= intersection_min
                    && y <= intersection_max
                {
                    intersections += 1;
                }
            }
        }
    }
    return intersections.to_string();
}

fn part_2(_input: &str) -> String {
    let output = Command::new("python3")
        .arg("./src/bin/year2023_day24.py")
        .output()
        .unwrap();

    format!("{}", String::from_utf8_lossy(&output.stdout))
        .trim()
        .to_owned()
}

fn main() {
    let input = read_input("2023", "24");

    let start_part_1 = Instant::now();
    let part_1_result = part_1(&input, 200000000000000.0, 400000000000000.0);
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
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
        "
        .trim();

        assert_eq!(part_1(input, 7.0, 27.0), "2");
    }

    #[ignore]
    #[test]
    fn test_part_2() {
        let input = r"".trim();

        assert_eq!(part_2(input), "");
    }
}
