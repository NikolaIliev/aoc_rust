use std::time::Instant;

use aoc_rust::read_input;
use itertools::Itertools;

struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

struct Game {
    id: usize,
    rounds: Vec<Round>,
}

impl Round {
    // Format:
    // 8 green, 6 blue, 20 red
    // 5 green, 3 red
    // 12 blue
    fn from_str(s: &str) -> Round {
        s.split(", ").fold(
            Round {
                red: 0,
                green: 0,
                blue: 0,
            },
            |mut round, part| {
                let (count_str, color) = part.split_once(" ").unwrap();
                let count = count_str.parse::<usize>().unwrap();

                match color.chars().next().unwrap() {
                    'r' => round.red = count,
                    'g' => round.green = count,
                    'b' => round.blue = count,
                    _ => (),
                }

                round
            },
        )
    }
}

impl Game {
    // Format:
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    fn from_str(s: &str) -> Game {
        let (game, rounds) = s.split_once(": ").unwrap();
        let (_, id) = game.split_once(" ").unwrap();
        let rounds: Vec<Round> = rounds.split("; ").map(Round::from_str).collect_vec();

        Game {
            id: id.parse::<usize>().unwrap(),
            rounds,
        }
    }
}

fn part_1(input: &str) -> String {
    let max_round = Round {
        red: 12,
        green: 13,
        blue: 14,
    };

    return input
        .lines()
        .map(Game::from_str)
        .filter_map(|game| {
            if game.rounds.iter().all(|round| {
                round.red <= max_round.red
                    && round.green <= max_round.green
                    && round.blue <= max_round.blue
            }) {
                Some(game.id)
            } else {
                None
            }
        })
        .sum::<usize>()
        .to_string();
}

fn part_2(input: &str) -> String {
    return input
        .lines()
        .map(Game::from_str)
        .map(|game| Round {
            red: game.rounds.iter().map(|round| round.red).max().unwrap(),
            green: game.rounds.iter().map(|round| round.green).max().unwrap(),
            blue: game.rounds.iter().map(|round| round.blue).max().unwrap(),
        })
        .map(|round| round.red * round.green * round.blue)
        .sum::<usize>()
        .to_string();
}

fn main() {
    let input = read_input("2023", "02");

    let start_part_1 = Instant::now();
    let part_1_result = part_1(&input);
    let part_1_time = start_part_1.elapsed();

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
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .trim();

        assert_eq!(part_1(input), "8");
    }

    #[test]
    fn test_part_2() {
        let input = r"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .trim();

        assert_eq!(part_2(input), "2286");
    }
}
