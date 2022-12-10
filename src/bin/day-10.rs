use aoc2022::commons::{io::load_argv_lines, grid::{BitGrid, Grid}};
use std::{error::Error, str::FromStr, num::ParseIntError};
use thiserror::Error;

#[derive(Debug)]
enum Instruction {
    AddX(isize),
    Noop,
}

impl Instruction {
    pub fn cycle_count(&self) -> usize {
        match self {
            Self::AddX(_) => 2,
            Self::Noop => 1,
        }
    }
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("Unknown instruction")]
    UnknownInstruction,
    #[error("Missing operand")]
    MissingOperand,
    #[error("Parse fail")]
    IntParseFail(#[from] ParseIntError),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = s.split(' ');
        match i.next() {
            Some("noop") => Ok(Instruction::Noop),
            Some("addx") => {
                let x = i.next().ok_or(ParseError::MissingOperand)?.parse()?;
                Ok(Instruction::AddX(x))
            }
            _ => Err(ParseError::UnknownInstruction),
        }
    }
}


fn part1(input: &[Instruction]) -> isize {
    let mut x = 1;
    let mut x_values = Vec::new();

    for i in input {
        for _ in 0..i.cycle_count() {
            x_values.push(x);
        }
        if let Instruction::AddX(add) = i {
            x += add;
        }
    }

    [20, 60, 100, 140, 180, 220].iter().map(|i| {
        let v = x_values[*i - 1];
        *i as isize * v
    }).sum()
}

fn part2(input: &[Instruction]) -> String {
    let mut x_reg = 1;
    let mut clock = 0;
    let mut grid = BitGrid::new(40, 6);

    for ins in input.iter() {
        for _ in 0..ins.cycle_count() {
            let y = clock / 40;
            let x = clock % 40;
            let set = x_reg - 1 == x as isize || x_reg == x as isize || x_reg + 1 == x as isize;
            grid.set((x, y), set);
            clock += 1;
        }
        if let Instruction::AddX(add) = ins {
            x_reg += add;
        }
    }

    grid.decode_string()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_argv_lines::<Instruction>().collect::<Result<Vec<_>, _>>()?;

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
                input_path: "inputs/10",
                part1_expected: 12560,
                part2_expected: "PLPAFBCL".to_string(),
            },
        ];

        for case in cases {
            let input = case
                .load_file()
                .lines()
                .map(|s| s.parse::<Instruction>().unwrap())
                .collect::<Vec<_>>();
            assert_eq!(part1(&input), case.part1_expected);
            assert_eq!(part2(&input), case.part2_expected);
        }
    }
}
