use std::time::Instant;

use aoc_rust::read_input;
use itertools::Itertools;

const CARD_TYPES_COUNT: usize = 13;
const CARD_RANKS_COUNT: usize = 7;

fn quantize_card(card: char, joker: bool) -> usize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if joker {
                1
            } else {
                11
            }
        }
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 0,
    }
}

fn quantize_hand(hand: &str, joker: bool) -> usize {
    let mut card_groups = hand.chars().counts();

    if joker {
        let jokers_count = *card_groups.get(&'J').unwrap_or(&0);

        if jokers_count < 5 {
            card_groups.remove(&'J');

            // to achieve the highest power we need the highest number of duplicate cards
            // so the strategy is to add the jokers to the group with the highest count
            let (max_card, max_card_count) =
                card_groups.iter().max_by_key(|&(_, count)| *count).unwrap();

            card_groups.insert(*max_card, *max_card_count + jokers_count);
        }
    }

    let power = match card_groups.len() {
        // 5 unique cards -> "High card"
        5 => 0,
        // 4 unique cards -> "One pair"
        4 => 1,
        // 3 unique cards -> Could be "Two pair" or "Three pair"
        3 => {
            if card_groups.iter().any(|(_, count)| *count == 3) {
                // One group has 3 of the same card -> must be "Three pair"
                3
            } else {
                // "Two pair"
                2
            }
        }
        // 2 unique cards -> Could be "Four of a kind" or "Full house"
        2 => match card_groups.iter().next().unwrap().1 {
            // four of a kind
            1 => 5,
            4 => 5,
            // must be full house
            _ => 4,
        },
        1 => 6,
        _ => 0,
    };

    let hand_sum = hand
        .chars()
        .map(|card| quantize_card(card, joker))
        .enumerate()
        .map(|(idx, card_score)| (CARD_TYPES_COUNT + 1).pow(5 - idx as u32) as usize * card_score)
        .sum::<usize>();

    (CARD_RANKS_COUNT + CARD_TYPES_COUNT + 1).pow(power + 5) + hand_sum
}

fn part_1(input: &str) -> String {
    input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(hand, bid_str)| {
            (
                (hand, quantize_hand(hand, false)),
                bid_str.parse::<usize>().unwrap(),
            )
        })
        .sorted_by_key(|((_, hand_score), _)| *hand_score)
        .enumerate()
        .map(|(idx, (_, bid))| (idx + 1) * bid)
        .sum::<usize>()
        .to_string()
}

fn part_2(input: &str) -> String {
    input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(hand, bid_str)| {
            (
                (hand, quantize_hand(hand, true)),
                bid_str.parse::<usize>().unwrap(),
            )
        })
        .sorted_by_key(|((_, hand_score), _)| *hand_score)
        .enumerate()
        .map(|(idx, (_, bid))| (idx + 1) * bid)
        .sum::<usize>()
        .to_string()
}

fn main() {
    let input = read_input("2023", "07");

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
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
        "
        .trim();

        assert_eq!(part_1(input), "6440");
    }

    #[test]
    fn test_part_2() {
        let input = r"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
        "
        .trim();

        assert_eq!(part_2(input), "5905");
    }
}
