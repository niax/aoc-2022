use aoc2022::commons::io::load_argv_lines;
use itertools::Itertools;
use std::error::Error;
use regex::Regex;

fn part1(input: &[u32]) -> u32 {
    0
}

fn part2(input: &[u32]) -> u32 {
    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut stack_strs = Vec::new();
    let mut moves = Vec::new();
    let mut do_moves = false;
    for l in load_argv_lines().collect::<Result<Vec<String>, _>>()? {
        if l.is_empty() {
            do_moves = true;
            continue;
        }
        if do_moves {
            moves.push(l);
        } else {
            stack_strs.push(l);
        }
    }

    let stack_count = (stack_strs[0].len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(stack_count);
    for _ in 0..stack_count {
        stacks.push(Vec::new());
    }
    stack_strs.pop();
    for stack_def in stack_strs {
        let mut i = 0;
        for mut c in &stack_def.chars().chunks(4) {
            match c.nth(1) {
                Some(' ') => {},
                Some(x) => stacks[i].insert(0, x),
                None => panic!("Oh no!"),
            }
            i+=1;
        }
    }

    let mut part1_stacks = stacks.clone();
    let mut part2_stacks = stacks.clone();

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)")?;
    for instruction in moves {
        let groups = re.captures(&instruction).unwrap();
        let n = groups.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let from = groups.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let to = groups.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;

        let mut new_entries = Vec::with_capacity(n);
        for _ in 0..n {
            let c = part1_stacks[from].pop().unwrap();
            part1_stacks[to].push(c);
            new_entries.insert(0, part2_stacks[from].pop().unwrap());
        }
        part2_stacks[to].extend(new_entries);

        println!("{} {} {}", n, from, to);
    }

    let mut s = String::new();
    for stack in part1_stacks {
        s += &stack.last().unwrap().to_string();
    }
    println!("{}", s);

    let mut s = String::new();
    for stack in part2_stacks {
        s += &stack.last().unwrap().to_string();
    }
    println!("{}", s);
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
                part1_expected: 0,
                part2_expected: 0,
            },
            //TestCase {
                //input_path: "inputs/04",
                //part1_expected: 567,
                //part2_expected: 907,
            //},
        ];

        for case in cases {
            let s = case.load_file();
            let input: Vec<u32> = s.lines().map(|l| l.parse().unwrap()).collect();

            assert_eq!(part1(&input), case.part1_expected);
            assert_eq!(part2(&input), case.part2_expected);
        }
    }
}
