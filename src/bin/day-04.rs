use aoc2022::commons::io::load_argv_lines;
use std::ops::RangeInclusive;
use std::error::Error;
use std::str::FromStr;
use regex::Regex;
use thiserror::Error;

#[derive(Debug, Error, Hash, Eq, PartialEq)]
enum ParseError {
    #[error("Bad item")]
    BadItem,
}

#[derive(Debug, Error, Hash, Eq, PartialEq)]
enum RunError {
    #[error("No matches")]
    NoMatch,
}

#[derive(Debug)]
struct Assignment {
    first: RangeInclusive<u8>,
    second: RangeInclusive<u8>,
}

impl Assignment {
    pub fn fully_contains(&self) -> bool {
        (self.first.contains(&self.second.start()) && self.first.contains(&self.second.end())) ||
            (self.second.contains(&self.first.start()) && self.second.contains(&self.first.end()))
    }

    pub fn any_overlap(&self) -> bool {
        (self.first.contains(&self.second.start()) || self.first.contains(&self.second.end())) ||
            (self.second.contains(&self.first.start()) || self.second.contains(&self.first.end()))
    }
}

impl FromStr for Assignment {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Statics, errors
        let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
        let captures = re.captures(s).unwrap();
        Ok(Assignment{
            first: captures.get(1).unwrap().as_str().parse().unwrap()..=captures.get(2).unwrap().as_str().parse().unwrap(),
            second: captures.get(3).unwrap().as_str().parse().unwrap()..=captures.get(4).unwrap().as_str().parse().unwrap(),
        })
    }
}

fn part1(input: &[Assignment]) -> Result<usize, RunError> {
    Ok(input.iter().filter(|a| a.fully_contains()).count())
}

fn part2(input: &[Assignment]) -> Result<usize, RunError> {
    Ok(input.iter().filter(|a| a.any_overlap()).count())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<Assignment> = load_argv_lines().collect::<Result<_, _>>()?;

    println!("{:?}", input);
    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);

    Ok(())
}
