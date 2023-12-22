use std::time::Instant;

use aoc_rust::read_input;
use itertools::Itertools;
use priority_queue::priority_queue;
use priority_queue::PriorityQueue;

#[derive(Debug, Clone)]
struct Point3D {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug, Clone)]
struct Brick {
    start: Point3D,
    end: Point3D,
    supports: Vec<usize>,
    supported_by: Vec<usize>,
}

fn parse_point_3d(s: &str) -> Point3D {
    let (x, y, z) = s
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();

    Point3D { x, y, z }
}

fn point_in_range(p: usize, range: (usize, usize)) -> bool {
    p >= range.0 && p <= range.1
}

fn has_collision(brick_a: &Brick, brick_b: &Brick) -> bool {
    (point_in_range(brick_a.start.z, (brick_b.start.z, brick_b.end.z))
        || point_in_range(brick_a.end.z, (brick_b.start.z, brick_b.end.z))
        || point_in_range(brick_b.start.z, (brick_a.start.z, brick_a.end.z))
        || point_in_range(brick_b.end.z, (brick_a.start.z, brick_a.end.z)))
        && (point_in_range(brick_a.start.x, (brick_b.start.x, brick_b.end.x))
            || point_in_range(brick_a.end.x, (brick_b.start.x, brick_b.end.x))
            || point_in_range(brick_b.start.x, (brick_a.start.x, brick_a.end.x))
            || point_in_range(brick_b.end.x, (brick_a.start.x, brick_a.end.x)))
        && (point_in_range(brick_a.start.y, (brick_b.start.y, brick_b.end.y))
            || point_in_range(brick_a.end.y, (brick_b.start.y, brick_b.end.y))
            || point_in_range(brick_b.start.y, (brick_a.start.y, brick_a.end.y))
            || point_in_range(brick_b.end.y, (brick_a.start.y, brick_a.end.y)))
}

fn move_down(brick: &mut Brick) {
    if brick.start.z > 1 {
        brick.start.z -= 1;
        brick.end.z -= 1;
    }
}

fn move_up(brick: &mut Brick) {
    brick.start.z += 1;
    brick.end.z += 1;
}

fn parse_bricks(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("~").unwrap();

            Brick {
                start: parse_point_3d(start),
                end: parse_point_3d(end),
                supports: vec![],
                supported_by: vec![],
            }
        })
        .sorted_by_key(|brick| brick.start.z)
        .fold(
            (Vec::<Brick>::new(), 1),
            |(mut result, max_z_reached), mut brick| {
                // position just above max_z_reached
                let diff = brick.end.z - brick.start.z;
                brick.start.z = max_z_reached + 1;
                brick.end.z = brick.start.z + diff;

                loop {
                    let mut can_move_down = true;

                    move_down(&mut brick);

                    for i in (0..result.len()).rev() {
                        if result[i].end.z < brick.start.z {
                            continue;
                        }

                        let collision = has_collision(&brick, &result[i]);

                        if collision {
                            can_move_down = false;
                            let len = result.len();
                            result[i].supports.push(len);
                            brick.supported_by.push(i);
                        }
                    }

                    if !can_move_down {
                        // cannot move any further down (collided with some brick)
                        // revert and break
                        move_up(&mut brick);
                        break;
                    }

                    // reached bottom
                    if brick.start.z == 1 {
                        break;
                    }
                }

                let new_max_z_reached = max_z_reached.max(brick.end.z);

                result.push(brick);

                (result, new_max_z_reached)
            },
        )
        .0
}

fn part_1(input: &str) -> String {
    let bricks = parse_bricks(input);

    bricks
        .iter()
        .filter(|b| {
            b.supports
                .iter()
                .all(|&idx| bricks[idx].supported_by.len() > 1)
        })
        .count()
        .to_string()
}

fn part_2(input: &str) -> String {
    let bricks = parse_bricks(input);

    let mut sum = 0;

    for i in (0..bricks.len()).rev() {
        if bricks[i].supports.is_empty() {
            continue;
        }

        let mut fallen_indexes: Vec<usize> = vec![];
        let mut pq = PriorityQueue::<usize, usize>::new();

        pq.push(i, bricks[i].end.z);

        while !pq.is_empty() {
            let (idx, _) = pq.pop().unwrap();

            fallen_indexes.push(idx);

            let brick = &bricks[idx];

            for supported_brick_idx in &brick.supports {
                let supported_brick = &bricks[*supported_brick_idx];

                if supported_brick
                    .supported_by
                    .iter()
                    .all(|idx| fallen_indexes.iter().rev().contains(idx))
                {
                    pq.push(*supported_brick_idx, supported_brick.end.z);
                }
            }
        }

        sum += fallen_indexes.len() - 1;
    }

    sum.to_string()
}

fn main() {
    let input = read_input("2023", "22");

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
1,0,1~1,2,1
0,2,3~2,2,3
2,0,5~2,2,5
0,0,4~0,2,4
0,1,6~2,1,6
0,0,2~2,0,2
1,1,8~1,1,9
        "
        .trim();

        assert_eq!(part_1(input), "5");
    }

    #[test]
    fn test_part_1_1() {
        let input = r"
0,0,1~0,0,1
0,0,3~0,0,3
        "
        .trim();

        assert_eq!(part_1(input), "1");
    }

    #[test]
    fn test_part_1_2() {
        let input = r"
0,0,1~0,0,1
0,1,3~0,1,3
        "
        .trim();

        assert_eq!(part_1(input), "2");
    }

    #[test]
    fn test_part_1_3() {
        let input = r"
0,0,1~0,0,10
0,0,300~0,1,300
        "
        .trim();

        assert_eq!(part_1(input), "1");
    }

    #[test]
    fn test_part_1_4() {
        let input = r"
0,0,1~0,0,10
0,0,300~0,1,300
1,0,50~1,0,50
        "
        .trim();

        assert_eq!(part_1(input), "2");
    }

    #[test]
    fn test_part_1_5() {
        let input = r"
0,0,1~0,0,10
0,1,1~0,1,10
0,0,300~0,1,300
        "
        .trim();

        assert_eq!(part_1(input), "3");
    }

    #[test]
    fn test_part_2() {
        let input = r"
1,0,1~1,2,1
0,2,3~2,2,3
2,0,5~2,2,5
0,0,4~0,2,4
0,1,6~2,1,6
0,0,2~2,0,2
1,1,8~1,1,9
        "
        .trim();

        assert_eq!(part_2(input), "7");
    }
}
