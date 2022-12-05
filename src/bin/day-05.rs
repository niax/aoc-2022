use aoc2022::commons::io::load_argv_lines;
use std::error::Error;

peg::parser! {
    grammar shipment_parser() for str {
        rule number() -> usize
            = n:$(['0'..='9']+) {? n.parse().or(Err("bad number")) }

        rule present_shipping_crate() -> Option<char>
            = "[" c:$(['A'..='Z']) "]" { c.chars().next() }
        rule missing_shipping_crate() -> Option<char>
            = "   " { None }

        rule shipping_crate() -> Option<char>
            = present_shipping_crate() / missing_shipping_crate()

        rule shipping_crates_entry() -> Option<char>
            = s:shipping_crate() " "  {
                s
            }

        pub rule shipping_crates() -> Vec<Option<char>>
            = l:shipping_crates_entry()* e:shipping_crate() {
                let mut v = l;
                v.push(e);
                v
            }

        pub rule move_instruction() -> MoveInstruction
            = "move " count:number() " from " from:number() " to " to:number() {
                MoveInstruction { count, from: from-1, to: to-1 }
            }
    }
}

#[derive(Debug, Clone)]
pub struct MoveInstruction {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, Clone)]
struct PuzzleInput {
    stacks: Vec<Vec<char>>,
    instructions: Vec<MoveInstruction>,
}

impl PuzzleInput {
    pub fn from_lines(lines: &[String]) -> PuzzleInput {
        // Find the point that ends stack defs and moves to instructions
        let split_point = lines.iter().position(|s| s.is_empty()).unwrap();
        let stack_count = (lines[0].len() + 1) / 4;
        let mut stacks: Vec<Vec<char>> = (0..stack_count)
            .map(|_| Vec::with_capacity(split_point))
            .collect();
        for l in &lines[0..split_point - 1] {
            for (i, c_opt) in shipment_parser::shipping_crates(l)
                .unwrap()
                .into_iter()
                .enumerate()
            {
                if let Some(c) = c_opt {
                    // TODO: Make this suck less
                    stacks[i].insert(0, c);
                }
            }
        }

        let instructions = lines[split_point + 1..]
            .iter()
            .map(|s| shipment_parser::move_instruction(s))
            .collect::<Result<_, _>>()
            .unwrap();

        PuzzleInput {
            stacks,
            instructions,
        }
    }
}

fn answer(stacks: &[Vec<char>]) -> String {
    let mut s = String::new();
    for stack in stacks {
        s += &stack.last().unwrap().to_string();
    }
    s
}

fn part1(input: &PuzzleInput) -> String {
    let mut stacks = input.stacks.clone();
    for instruction in &input.instructions {
        for _ in 0..instruction.count {
            let c = stacks[instruction.from].pop().unwrap();
            stacks[instruction.to].push(c);
        }
    }

    answer(&stacks)
}

fn part2(input: &PuzzleInput) -> String {
    let mut stacks = input.stacks.clone();
    for instruction in &input.instructions {
        let mut new_entries = Vec::with_capacity(instruction.count);
        for _ in 0..instruction.count {
            new_entries.push(stacks[instruction.from].pop().unwrap());
        }
        new_entries.reverse();
        stacks[instruction.to].extend(new_entries);
    }

    answer(&stacks)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = PuzzleInput::from_lines(&load_argv_lines().collect::<Result<Vec<String>, _>>()?);

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
                input_path: "inputs/extra/05.sample",
                part1_expected: "CMZ".to_string(),
                part2_expected: "MCD".to_string(),
            },
            TestCase {
                input_path: "inputs/05",
                part1_expected: "FRDSQRRCD".to_string(),
                part2_expected: "HRFTQVWNN".to_string(),
            },
        ];

        for case in cases {
            let s = case
                .load_file()
                .lines()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            let input = PuzzleInput::from_lines(&s);

            assert_eq!(part1(&input), case.part1_expected);
            assert_eq!(part2(&input), case.part2_expected);
        }
    }
}
