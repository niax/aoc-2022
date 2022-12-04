use std::str::FromStr;
use thiserror::Error;

use aoc2022::commons::io::load_argv_lines;

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    pub fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    pub fn base_score(&self) -> u32 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }

    pub fn outcome(&self, other: &Play) -> Outcome {
        if self == other {
            Outcome::Draw
        } else {
            let win = match self {
                Play::Rock => *other == Play::Scissors,
                Play::Paper => *other == Play::Rock,
                Play::Scissors => *other == Play::Paper,
            };

            if win {
                Outcome::Win
            } else {
                Outcome::Lose
            }
        }
    }

    pub fn play_desired_for_outcome(&self, outcome: &Outcome) -> Play {
        match *outcome {
            Outcome::Lose => match *self {
                Play::Rock => Play::Scissors,
                Play::Paper => Play::Rock,
                Play::Scissors => Play::Paper,
            },
            Outcome::Draw => *self,
            Outcome::Win => match *self {
                Play::Rock => Play::Paper,
                Play::Paper => Play::Scissors,
                Play::Scissors => Play::Rock,
            },
        }
    }
}

#[derive(Debug)]
struct Round {
    opponent: Play,
    mine: Play,
    intended_outcome: Outcome,
}

impl Round {
    pub fn score(&self) -> u32 {
        self.mine.outcome(&self.opponent).score() + self.mine.base_score()
    }

    pub fn intended_score(&self) -> u32 {
        let play_desired_for_outcome = self
            .opponent
            .play_desired_for_outcome(&self.intended_outcome);
        play_desired_for_outcome.base_score() + self.intended_outcome.score()
    }
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Unknown symbol")]
    UnknownSymbol,

    #[error("Parse error")]
    ParseError,
}

impl FromStr for Round {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.chars();
        let opponent = match it.next() {
            Some('A') => Play::Rock,
            Some('B') => Play::Paper,
            Some('C') => Play::Scissors,
            Some(_) => return Err(ParseError::UnknownSymbol),
            _ => return Err(ParseError::ParseError),
        };

        match it.next() {
            Some(' ') => {}
            _ => return Err(ParseError::ParseError),
        }

        let col2 = it.next();
        let mine = match col2 {
            Some('X') => Play::Rock,
            Some('Y') => Play::Paper,
            Some('Z') => Play::Scissors,
            Some(_) => return Err(ParseError::UnknownSymbol),
            _ => return Err(ParseError::ParseError),
        };

        let intended_outcome = match col2 {
            Some('X') => Outcome::Lose,
            Some('Y') => Outcome::Draw,
            Some('Z') => Outcome::Win,
            Some(_) => return Err(ParseError::UnknownSymbol),
            _ => return Err(ParseError::ParseError),
        };

        Ok(Round {
            opponent,
            mine,
            intended_outcome,
        })
    }
}

fn part1(input: &[Round]) -> u32 {
    input.iter().map(|r| r.score()).sum()
}

fn part2(input: &[Round]) -> u32 {
    input.iter().map(|r| r.intended_score()).sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Vec<Round> = load_argv_lines().collect::<Result<_, _>>()?;

    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use super::*;

    struct TestCase {
        input_path: &'static str,
        part1_expected: u32,
        part2_expected: u32,
    }

    #[test]
    fn test_solution() {
        let cases = [
            TestCase {
                input_path: "inputs/extra/02.sample",
                part1_expected: 15,
                part2_expected: 12,
            },
            TestCase {
                input_path: "inputs/02",
                part1_expected: 13565,
                part2_expected: 12424,
            },
        ];

        for case in cases {
            let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            input_path.push(case.input_path);
            let s = fs::read_to_string(input_path).unwrap();
            let input: Vec<Round> = s.lines().map(|l| l.parse().unwrap()).collect();

            assert_eq!(part1(&input), case.part1_expected);
            assert_eq!(part2(&input), case.part2_expected);
        }
    }
}
