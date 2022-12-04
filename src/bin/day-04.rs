use aoc2022::commons::io::load_argv_lines;
use peg::str::LineCol;
use std::error::Error;
use std::ops::RangeInclusive;
use std::str::FromStr;
use peg;

peg::parser! {
    grammar assignment_parser() for str {
        rule number() -> u8
            = n:$(['0'..='9']+) {? n.parse().or(Err("bad number")) }

        pub rule assignment() -> Assignment
            = n1:number() "-" n2:number() "," n3:number() "-" n4:number() {
                Assignment { first: (n1..=n2), second: (n3..=n4) }
            }
    }

}

#[derive(Debug)]
pub struct Assignment {
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
    type Err = peg::error::ParseError<LineCol>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assignment_parser::assignment(s)
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
