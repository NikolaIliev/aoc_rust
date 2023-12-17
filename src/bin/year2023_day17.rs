use std::time::Instant;

use aoc_rust::read_input;
use itertools::Itertools;
use priority_queue::PriorityQueue;

fn is_valid_location(matrix: &Vec<Vec<u8>>, location: (isize, isize)) -> bool {
    location.0 >= 0
        && (location.0 as usize) < matrix[0].len()
        && location.1 >= 0
        && (location.1 as usize) < matrix.len()
}

fn quantize_direction(direction: (i8, i8)) -> usize {
    match direction {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => unreachable!(),
    }
}

fn solve(input: &str, skip: u8, max_forward: u8) -> String {
    let matrix: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|ch| ch as u8 - '0' as u8).collect_vec())
        .collect_vec();

    // cache for each pos and direction
    let mut min_heat_losses: Vec<Vec<[usize; 4]>> =
        vec![vec![[usize::MAX, usize::MAX, usize::MAX, usize::MAX]; matrix[0].len()]; matrix.len()];

    // items are (pos, dir), cost is the total heat_loss * -1 (we prioritise lower losses)
    // cba to impl a custom struct so -1 it is
    let mut pq = PriorityQueue::<((isize, isize), (i8, i8)), isize>::new();

    pq.push(((0, 0), (1, 0)), 0);
    pq.push(((0, 0), (0, 1)), 0);

    while !pq.is_empty() {
        let (((x, y), direction), heat_loss) = pq.pop().unwrap();

        if x as usize == matrix[0].len() - 1 && y as usize == matrix.len() - 1 {
            return min_heat_losses[y as usize][x as usize]
                .iter()
                .min()
                .unwrap()
                .to_string();
        }

        // go left/right or top/bottom, depending on where the node was enqueued from
        // left/right -> go top/bottom
        // top/bottom -> go left/right
        for j in [-1, 1] {
            // as we're directly adding multiple in one direction,
            // we need to accumulate their heat_loss to enqueue with a correct prio
            let mut new_loc_heat_loss_acc = 0;
            // enqueue up to the max forward nodes in both directions
            for i in 1..=max_forward {
                let new_dir = (direction.0 * j, direction.1 * j);
                let new_loc = (
                    x + (new_dir.0 * i as i8) as isize,
                    y + (new_dir.1 * i as i8) as isize,
                );

                if !is_valid_location(&matrix, new_loc) {
                    // if this one is not valid (out of matrix) then the rest won't be too
                    break;
                }

                new_loc_heat_loss_acc += matrix[new_loc.1 as usize][new_loc.0 as usize] as isize;

                if i < skip {
                    continue;
                }

                let new_loc_heat_loss = -heat_loss + new_loc_heat_loss_acc;

                if (new_loc_heat_loss as usize)
                    <= min_heat_losses[new_loc.1 as usize][new_loc.0 as usize]
                        [quantize_direction(new_dir)]
                {
                    min_heat_losses[new_loc.1 as usize][new_loc.0 as usize]
                        [quantize_direction(new_dir)] = new_loc_heat_loss as usize;

                    pq.push((new_loc, (new_dir.1, new_dir.0)), -new_loc_heat_loss);
                }
            }
        }
    }

    unreachable!()
}

fn part_1(input: &str) -> String {
    solve(input, 0, 3)
}

fn part_2(input: &str) -> String {
    solve(input, 4, 10)
}

fn main() {
    let input = read_input("2023", "17");

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
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
        "
        .trim();

        assert_eq!(part_1(input), "102");
    }

    #[ignore]
    #[test]
    fn test_part_2() {
        let input = r"".trim();

        assert_eq!(part_2(input), "");
    }
}
