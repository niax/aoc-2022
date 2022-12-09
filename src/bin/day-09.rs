use aoc2022::commons::geom::Point;
use aoc2022::commons::grid::{BitGrid, Grid};
use aoc2022::commons::io::load_argv_lines;
use std::error::Error;

fn solve(input: &[Instruction]) -> (usize, usize) {
    let mut elems = (0..=9)
        .map(|_| Point::new(300_isize, 300_isize))
        .collect::<Vec<Point<isize>>>();
    let mut part1 = BitGrid::new(600, 600);
    let mut part2 = BitGrid::new(600, 600);

    for instruction in input {
        for _ in 0..instruction.n() {
            let mut last = 0;
            elems[last] += instruction.as_dir();
            for knot in 1..=9 {
                let last_knot = elems[last];
                let mut tail = elems[knot];
                let diff = last_knot - elems[knot];
                if diff.x() == &0 || diff.y() == &0 {
                    if diff.x().abs() >= 2 {
                        tail += (diff.x().signum(), 0);
                    }
                    if diff.y().abs() >= 2 {
                        tail += (0, diff.y().signum());
                    }
                } else if !(diff.x().abs() == 1 && diff.y().abs() == 1) {
                    tail += (diff.x().signum(), diff.y().signum());
                }
                elems[knot] = tail;
                last = knot;
            }

            part1.set((*elems[1].x() as usize, *elems[1].y() as usize), true);
            part2.set((*elems[9].x() as usize, *elems[9].y() as usize), true);
        }
    }

    (part1.set_cell_count(), part2.set_cell_count())
}

#[derive(Debug)]
enum Instruction {
    Up(isize),
    Left(isize),
    Down(isize),
    Right(isize),
}

impl Instruction {
    fn as_dir(&self) -> (isize, isize) {
        match self {
            Instruction::Up(_) => (0, 1),
            Instruction::Left(_) => (-1, 0),
            Instruction::Down(_) => (0, -1),
            Instruction::Right(_) => (1, 0),
        }
    }

    fn n(&self) -> isize {
        match self {
            Instruction::Up(n) => *n,
            Instruction::Left(n) => *n,
            Instruction::Down(n) => *n,
            Instruction::Right(n) => *n,
        }
    }
}

fn parse(input: &[String]) -> Vec<Instruction> {
    input
        .iter()
        .map(|l| {
            let mut s = l.split(' ');
            let first = s.next().unwrap();
            let second = s.next().unwrap().parse().unwrap();
            match first {
                "U" => Instruction::Up(second),
                "L" => Instruction::Left(second),
                "D" => Instruction::Down(second),
                "R" => Instruction::Right(second),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<_>>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_argv_lines::<String>().collect::<Result<Vec<_>, _>>()?;
    let instructions = parse(&input);

    let (part1, part2) = solve(&instructions);
    println!("{}", part1);
    println!("{}", part2);

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
                input_path: "inputs/extra/09.sample",
                part1_expected: 13,
                part2_expected: 1,
            },
            TestCase {
                input_path: "inputs/09",
                part1_expected: 6030,
                part2_expected: 2545,
            },
        ];

        for case in cases {
            let input = case
                .load_file()
                .lines()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            let instructions = parse(&input);
            let (part1, part2) = solve(&instructions);
            assert_eq!(part1, case.part1_expected);
            assert_eq!(part2, case.part2_expected);
        }
    }
}
