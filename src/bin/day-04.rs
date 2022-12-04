use aoc2022::commons::io::load_argv_lines;
use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;
use std::ops::RangeInclusive;
use std::str::FromStr;
use thiserror::Error;

lazy_static! {
    static ref PARSE_REGEX: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
}

#[derive(Debug, Error, Eq, PartialEq)]
enum ParseError {
    #[error("Bad line")]
    BadLine,
    #[error("Bad number")]
    BadNumber(#[from] std::num::ParseIntError),
}

#[derive(Debug)]
struct Assignment {
    first: RangeInclusive<u8>,
    second: RangeInclusive<u8>,
}

impl Assignment {
    pub fn fully_contains(&self) -> bool {
        (self.first.contains(self.second.start()) && self.first.contains(self.second.end()))
            || (self.second.contains(self.first.start()) && self.second.contains(self.first.end()))
    }

    pub fn any_overlap(&self) -> bool {
        (self.first.contains(self.second.start()) || self.first.contains(self.second.end()))
            || (self.second.contains(self.first.start()) || self.second.contains(self.first.end()))
    }
}

impl FromStr for Assignment {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: errors
        let captures = PARSE_REGEX.captures(s).ok_or(ParseError::BadLine)?;
        let numbers: Vec<u8> = captures
            .iter()
            .skip(1) // Ignore group 0
            .map(|group| match group {
                Some(g) => g.as_str().parse::<u8>().map_err(ParseError::from),
                None => Err(ParseError::BadLine),
            })
            .collect::<Result<_, _>>()?;

        Ok(Assignment {
            first: numbers[0]..=numbers[1],
            second: numbers[2]..=numbers[3],
        })
    }
}

fn part1(input: &[Assignment]) -> usize {
    input.iter().filter(|a| a.fully_contains()).count()
}

fn part2(input: &[Assignment]) -> usize {
    input.iter().filter(|a| a.any_overlap()).count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<Assignment> = load_argv_lines().collect::<Result<_, _>>()?;

    println!("{}", part1(&input));
    println!("{}", part2(&input));

    Ok(())
}
