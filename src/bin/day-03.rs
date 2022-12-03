use thiserror::Error;
use aoc2022::commons::io::load_argv_lines;
use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;

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
    c1: HashSet<usize>,
    c2: HashSet<usize>,
    all: HashSet<usize>,
}

impl Bag {
    pub fn badge(&self) -> Option<usize> {
        let mut inter = self.c1.intersection(&self.c2);
        inter.next().copied()
    }
}

impl FromStr for Bag {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s1, s2) = s.split_at(s.len() / 2);
        Ok(Bag {
            c1: s1.chars().map(|c| priority_for_char(c)).collect::<Result<_, _>>()?,
            c2: s2.chars().map(|c| priority_for_char(c)).collect::<Result<_, _>>()?,
            all: s.chars().map(|c| priority_for_char(c)).collect::<Result<_, _>>()?,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<Bag> = load_argv_lines().map(|res| res).collect::<Result<_, _>>()?;

    let part1: usize = input.iter().map(|b| b.badge().unwrap()).sum();
    let part2: usize = input.chunks_exact(3).map(|window| {
        window[0].all.intersection(&window[1].all).find(|priority| window[2].all.contains(priority)).ok_or(RunError::NoMatch)
    }).sum::<Result<_, _>>()?;


    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}
