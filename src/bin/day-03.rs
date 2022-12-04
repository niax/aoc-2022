use aoc2022::commons::io::load_argv_lines;
use bitvec::prelude::*;
use std::error::Error;
use std::str::FromStr;
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

fn priority_for_char(ch: char) -> Result<usize, ParseError> {
    let c = ch as usize;
    match ch {
        'A'..='Z' => Ok(c - 64 + 26),
        'a'..='z' => Ok(c - 96),
        _ => Err(ParseError::BadItem),
    }
}

#[derive(Debug)]
struct Bag {
    c1: BitArray<[u64; 1]>,
    c2: BitArray<[u64; 1]>,
    all: BitArray<[u64; 1]>,
}

impl Bag {
    pub fn badge(&self) -> Option<usize> {
        let inter = self.c1 & self.c2;
        inter.first_one()
    }
}

impl FromStr for Bag {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s1, s2) = s.split_at(s.len() / 2);
        let mut c1 = bitarr![u64, Lsb0; 0; 64];
        let mut c2 = bitarr![u64, Lsb0; 0; 64];
        let mut all = bitarr![u64, Lsb0; 0; 64];
        for c in s1.chars() {
            let p = priority_for_char(c)?;
            c1.set(p, true);
            all.set(p, true);
        }
        for c in s2.chars() {
            let p = priority_for_char(c)?;
            c2.set(p, true);
            all.set(p, true);
        }
        Ok(Bag { c1, c2, all })
    }
}

fn part1(input: &[Bag]) -> Result<usize, RunError> {
    input
        .iter()
        .map(|b| b.badge().ok_or(RunError::NoMatch))
        .sum::<Result<_, _>>()
}

fn part2(input: &[Bag]) -> Result<usize, RunError> {
    input
        .chunks_exact(3)
        .map(|window| {
            let inter = window[0].all & window[1].all & window[2].all;
            inter.first_one().ok_or(RunError::NoMatch)
        })
        .sum::<Result<_, _>>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<Bag> = load_argv_lines().collect::<Result<_, _>>()?;

    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2022::commons::test_helpers::TestCase;

    #[test]
    fn test_solution() {
        let cases = [
            TestCase {
                input_path: "inputs/extra/03.sample",
                part1_expected: 157,
                part2_expected: 70,
            },
            TestCase {
                input_path: "inputs/03",
                part1_expected: 8123,
                part2_expected: 2620,
            },
        ];

        for case in cases {
            let s = case.load_file();
            let input: Vec<Bag> = s.lines().map(|l| l.parse().unwrap()).collect();

            assert_eq!(part1(&input), Ok(case.part1_expected));
            assert_eq!(part2(&input), Ok(case.part2_expected));
        }
    }
}
