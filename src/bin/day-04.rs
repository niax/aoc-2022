use aoc2022::commons::io::load_argv_lines;
use peg::str::LineCol;
use std::error::Error;
use std::ops::RangeInclusive;
use std::str::FromStr;

peg::parser! {
    grammar assignment_parser() for str {
        rule number() -> u8
            = n:$(['0'..='9']+) {? n.parse().or(Err("bad number")) }

        rule range() -> RangeInclusive<u8>
            = start:number() "-" end:number() {
                start..=end
            }

        pub rule assignment() -> Assignment
            = first:range() "," second:range() {
                Assignment { first, second }
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

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2022::commons::test_helpers::TestCase;

    #[test]
    fn test_solution() {
        let cases = [
            TestCase {
                input_path: "inputs/extra/04.sample",
                part1_expected: 2,
                part2_expected: 4,
            },
            TestCase {
                input_path: "inputs/04",
                part1_expected: 567,
                part2_expected: 907,
            },
        ];

        for case in cases {
            let s = case.load_file();
            let input: Vec<Assignment> = s.lines().map(|l| l.parse().unwrap()).collect();

            assert_eq!(part1(&input), case.part1_expected);
            assert_eq!(part2(&input), case.part2_expected);
        }
    }
}
