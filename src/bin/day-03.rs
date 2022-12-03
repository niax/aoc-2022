use aoc2022::commons::io::load_argv_lines;
use std::error::Error;
use std::str::FromStr;
use thiserror::Error;
use bitvec::prelude::*;

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
    if (65..92).contains(&c) {
        Ok(c - 64 + 26)
    } else if (97..123).contains(&c) {
        Ok(c - 96)
    } else {
        Err(ParseError::BadItem)
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

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<Bag> = load_argv_lines().collect::<Result<_, _>>()?;

    let part1: usize = input
        .iter()
        .map(|b| b.badge().ok_or(RunError::NoMatch))
        .sum::<Result<_, _>>()?;
    let part2: usize = input
        .chunks_exact(3)
        .map(|window| {
            let inter = window[0].all & window[1].all & window[2].all;
            inter.first_one()
                .ok_or(RunError::NoMatch)
        })
        .sum::<Result<_, _>>()?;

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}
