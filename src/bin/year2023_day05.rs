use std::time::Instant;

use aoc_rust::read_input;
use itertools::Itertools;

fn parse_range(line: &str) -> Option<(usize, usize, usize)> {
    match line.chars().next() {
        None => None,
        Some(ch) => match ch {
            ch if ch.is_numeric() => {
                let mut iter = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());

                Some((
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                ))
            }

            _ => None,
        },
    }
}

fn part_1(input: &str) -> String {
    input
        .lines()
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .map(|seed| {
            input
                .lines()
                .skip(3)
                .map(parse_range)
                .fold((seed, false), |(n, skip), range| match range {
                    None => (n, false),
                    Some((destination, source, count)) => {
                        if skip {
                            (n, true)
                        } else {
                            if n >= source && n - source < count {
                                (destination + n - source, true)
                            } else {
                                (n, false)
                            }
                        }
                    }
                })
        })
        .map(|(n, _)| n)
        .min()
        .unwrap()
        .to_string()
}

#[derive(Debug)]
struct MapEntry {
    input_range: (usize, usize),
    output_range: Option<(usize, usize)>,
}

fn collide(
    source: &(usize, usize),
    target: &(usize, usize),
) -> (Option<(usize, usize)>, Vec<(usize, usize)>) {
    // no collision
    if source.0 < target.0 && source.1 <= target.0 || source.0 >= target.1 && source.1 > target.1 {
        return (None, vec![*source]);
    }

    // source is inside target
    if source.0 >= target.0 && source.1 <= target.1 {
        return (Some(*source), vec![]);
    }

    // target is inside source
    if target.0 >= source.0 && target.1 <= source.1 {
        return (
            Some(*target),
            vec![(source.0, target.0), (target.1, source.1)],
        );
    }

    // [s    {t s]  t}
    if source.0 < target.0 {
        return (
            Some((target.0, source.1)),
            vec![(source.0, target.0), (source.1, target.1)],
        );
    }

    //  {t s[  t}     s]
    if target.0 < source.0 {
        return (
            Some((source.0, target.1)),
            vec![(target.0, source.0), (target.1, source.1)],
        );
    }

    panic!("Wtf failed to collide")
}

fn traverse(
    maps: &Vec<Vec<MapEntry>>,
    maps_i: usize,
    range: (usize, usize),
    original_range: (usize, usize),
) -> Option<(usize, usize)> {
    if maps_i == 0 {
        for target_entry in &maps[0] {
            let (overlap_range, _) = collide(&range, &target_entry.input_range);

            if let Some(overlap_range) = overlap_range {
                return Some((
                    original_range.0 + (overlap_range.0 - range.0),
                    original_range.1 + (overlap_range.1 - range.1),
                ));
            }
        }

        return None;
    }

    let mut ranges_to_check: Vec<(usize, usize)> = vec![range];

    for target_entry in &maps[maps_i] {
        let (overlap_range, mut missed_ranges) = collide(&range, &target_entry.input_range);

        if let Some(overlap_range) = overlap_range {
            let found = traverse(
                maps,
                maps_i - 1,
                (
                    overlap_range.0 + target_entry.output_range.unwrap().0
                        - target_entry.input_range.0,
                    overlap_range.1 + target_entry.output_range.unwrap().0
                        - target_entry.input_range.0,
                ),
                (
                    original_range.0 + (overlap_range.0 - range.0),
                    original_range.1 + (overlap_range.1 - range.1),
                ),
            );

            if let Some(found) = found {
                return Some(found);
            }
        }

        ranges_to_check.append(&mut missed_ranges);
    }

    return None;
}

fn part_2(input: &str) -> String {
    let (seeds_str, maps_str) = input.split_once("\n").unwrap();
    let mut maps: Vec<Vec<MapEntry>> = Vec::new();

    maps.push(
        seeds_str
            .split_once(": ")
            .unwrap()
            .1
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect_vec()
            .chunks(2)
            .map(|slice| MapEntry {
                input_range: (slice[0], slice[0] + slice[1]),
                output_range: None,
            })
            .collect_vec(),
    );

    for map_str in maps_str.split("\n\n") {
        let mut entries = map_str
            .lines()
            .filter(|line| line.chars().next().unwrap_or(' ').is_numeric())
            .fold(Vec::<MapEntry>::new(), |mut ranges, line| {
                let mut iter = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
                let input_start = iter.next().unwrap();
                let output_start = iter.next().unwrap();
                let count = iter.next().unwrap();

                ranges.push(MapEntry {
                    input_range: (input_start, input_start + count),
                    output_range: Some((output_start, output_start + count)),
                });

                ranges
            });

        entries.sort_by(|a, b| a.output_range.unwrap().0.cmp(&b.output_range.unwrap().0));

        let smallest = entries.first().unwrap().output_range.unwrap().0;
        let largest = entries.last().unwrap().output_range.unwrap().1;

        if smallest > 0 {
            let mut new_entries = vec![MapEntry {
                input_range: (0, smallest),
                output_range: Some((0, smallest)),
            }];

            new_entries.append(&mut entries);
            new_entries.push(MapEntry {
                input_range: (largest, 999999999999),
                output_range: Some((largest, 999999999999)),
            });

            maps.push(new_entries);
        } else {
            entries.push(MapEntry {
                input_range: (largest, 999999999999),
                output_range: Some((largest, 999999999999)),
            });
            maps.push(entries)
        }
    }

    maps.last()
        .unwrap()
        .iter()
        .map(|entry| {
            traverse(
                &maps,
                maps.len() - 2,
                entry.output_range.unwrap(),
                entry.input_range,
            )
        })
        .filter(|found| found.is_some())
        .min_by_key(|found| found.unwrap().0)
        .unwrap()
        .unwrap()
        .0
        .to_string()
}

fn main() {
    let input = read_input("2023", "05");

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
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48
0 50 50

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70
0 18 18

light-to-temperature map:
45 77 23
81 45 19
68 64 13
0 45 45

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
        "
        .trim();

        assert_eq!(part_1(input), "35");
    }

    #[test]
    fn test_part_2() {
        let input = r"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48
0 0 50

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70
0 0 18

light-to-temperature map:
45 77 23
81 45 19
68 64 13
0 0 45

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
        "
        .trim();

        assert_eq!(part_2(input), "");
    }

    #[test]
    fn test_collide() {
        assert_eq!(collide(&(0, 5), &(6, 10)), (None, vec![(0, 5)]));
        assert_eq!(
            collide(&(0, 5), &(2, 3)),
            (Some((2, 3)), vec![(0, 2), (3, 5)])
        );
    }
}
