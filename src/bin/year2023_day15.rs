use std::time::Instant;

use aoc_rust::read_input;

fn hash_algo(s: &str) -> usize {
    s.chars()
        .fold(0, |val, ch| ((val + (ch as u8) as usize) * 17) % 256)
}

fn part_1(input: &str) -> String {
    input.split(",").map(hash_algo).sum::<usize>().to_string()
}

#[derive(Clone, Debug)]
struct Lens<'a> {
    id: &'a str,
    focal_length: usize,
}

fn part_2<'a>(input: &'a str) -> String {
    input
        .split(",")
        .fold(vec![Vec::<Lens<'a>>::new(); 256], |mut boxes, s| {
            let parts: Vec<&str> = s.split_terminator(['=', '-']).collect();
            let id = parts[0];
            let bx = &mut boxes[hash_algo(id)];
            let idx_of_lens =
                bx.iter()
                    .enumerate()
                    .find_map(|(idx, lens)| if lens.id == id { Some(idx) } else { None });

            if parts.len() == 2 {
                // =
                let id = parts[0];
                let focal_length = parts[1].parse::<usize>().unwrap();

                if idx_of_lens.is_some() {
                    bx[idx_of_lens.unwrap()].focal_length = focal_length;
                } else {
                    bx.push(Lens { id, focal_length });
                }
            } else {
                // -
                if idx_of_lens.is_some() {
                    bx.remove(idx_of_lens.unwrap());
                }
            }

            boxes
        })
        .iter()
        .enumerate()
        .fold(0, |sum, (box_idx, bx)| {
            sum + bx.iter().enumerate().fold(0, |sum, (lens_idx, lens)| {
                sum + (box_idx + 1) * (lens_idx + 1) * lens.focal_length
            })
        })
        .to_string()
}

fn main() {
    let input = read_input("2023", "15");

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
        let input = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".trim();

        assert_eq!(part_1(input), "1320");
    }

    #[test]
    fn test_part_2() {
        let input = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".trim();

        assert_eq!(part_2(input), "145");
    }
}
