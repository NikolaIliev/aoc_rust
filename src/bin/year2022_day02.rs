use aoc_rust::read_input;

enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_char(char: char) -> Shape {
        match char {
            'X' | 'A' => Shape::Rock,
            'Y' | 'B' => Shape::Paper,
            'Z' | 'C' => Shape::Scissors,
            _ => panic!("Unrecognized char"),
        }
    }

    fn score(&self) -> usize {
        match &self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn versus(&self, shape: &Shape) -> Outcome {
        match &self {
            Shape::Rock => match shape {
                Shape::Rock => Outcome::Draw,
                Shape::Paper => Outcome::Loss,
                Shape::Scissors => Outcome::Win,
            },

            Shape::Scissors => match shape {
                Shape::Rock => Outcome::Loss,
                Shape::Paper => Outcome::Win,
                Shape::Scissors => Outcome::Draw,
            },

            Shape::Paper => match shape {
                Shape::Rock => Outcome::Win,
                Shape::Paper => Outcome::Draw,
                Shape::Scissors => Outcome::Loss,
            },
        }
    }
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn from_char(char: char) -> Outcome {
        match char {
            'X' => Outcome::Loss,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("Unrecognized char"),
        }
    }

    fn score(&self) -> usize {
        match &self {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
        }
    }
}

fn part_1(input: &str) -> String {
    return input
        .lines()
        .map(|line| {
            let ours = Shape::from_char(line.chars().nth(2).unwrap());
            let theirs = Shape::from_char(line.chars().nth(0).unwrap());

            return ours.versus(&theirs).score() + ours.score();
        })
        .sum::<usize>()
        .to_string();
}

fn part_2(input: &str) -> String {
    impl Outcome {}

    return input
        .lines()
        .map(|line| {
            let theirs = Shape::from_char(line.chars().nth(0).unwrap());
            let outcome = Outcome::from_char(line.chars().nth(2).unwrap());

            let ours = match outcome {
                Outcome::Draw => theirs,
                Outcome::Win => match theirs {
                    Shape::Rock => Shape::Paper,
                    Shape::Paper => Shape::Scissors,
                    Shape::Scissors => Shape::Rock,
                },
                Outcome::Loss => match theirs {
                    Shape::Rock => Shape::Scissors,
                    Shape::Paper => Shape::Rock,
                    Shape::Scissors => Shape::Paper,
                },
            };

            return ours.score() + outcome.score();
        })
        .sum::<usize>()
        .to_string();
}

fn main() {
    let input = read_input("2022", "02");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r"
A Y
B X
C Z
        "
        .trim();

        assert_eq!(part_1(input), "15");
    }

    #[test]
    fn test_part_2() {
        let input = r"
A Y
B X
C Z
        "
        .trim();

        assert_eq!(part_2(input), "12");
    }
}
