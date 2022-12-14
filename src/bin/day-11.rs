use aoc2022::commons::io::get_argv_reader;
use std::{cell::RefCell, error::Error};

peg::parser! {
    grammar monkey_parser() for str {
        rule number() -> usize
            = n:$(['0'..='9']+) {? n.parse().or(Err("bad number")) }

        rule number_list() -> Vec<usize>
            = nums:(number() ** ", ")  {
                nums
            }

        rule var_num() -> Var =
            n: number() {
                Var::Literal(n)
        }

        rule var_old() -> Var =
            "old" {
                Var::Old
        }

        rule var() -> Var
            = var_num() / var_old()

        rule operation() -> Operation =
            precedence!{
                "+ " v:var() {Operation::Add(v)}
                "- " v:var() {Operation::Sub(v)}
                "* " v:var() {Operation::Mul(v)}
                "/ " v:var() {Operation::Div(v)}
            }

        rule operation_str() -> Operation =
            "  Operation: new = old " o:operation() { o }

        rule throw_rule() -> usize =
            "throw to monkey " n:number() { n }

        rule test() -> Test =
            "  Test: divisible by " div:number()  "\n"
                "    If true: " t:throw_rule() "\n"
                "    If false: " f:throw_rule()  "\n" {
                    Test {
                        divisible_by: div,
                        if_true: t,
                        if_false: f,
                    }
                }

        rule monkey() -> Monkey
            = "Monkey " number() ":\n"
                "  Starting items: " items:number_list() "\n"
                op:operation_str() "\n"
                test:test() {
                    Monkey {
                        items,
                        op,
                        test,
                        inspections: 0,
                    }
                }

        pub rule monkeys() -> Vec<Monkey>
            = monkey() ** "\n"

    }
}

#[derive(Debug, Clone)]
struct Test {
    divisible_by: usize,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug, Clone)]
pub enum Var {
    Literal(usize),
    Old,
}

impl Var {
    pub fn get(&self, old: usize) -> usize {
        match self {
            Self::Literal(n) => *n,
            Self::Old => old,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    Add(Var),
    Div(Var),
    Mul(Var),
    Sub(Var),
}

impl Operation {
    pub fn apply(&self, old: usize) -> usize {
        match self {
            Self::Add(v) => old + v.get(old),
            Self::Div(v) => old / v.get(old),
            Self::Mul(v) => old * v.get(old),
            Self::Sub(v) => old - v.get(old),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<usize>,
    op: Operation,
    test: Test,
    inspections: usize,
}

impl Monkey {
    fn step(&mut self, monkeys: &[RefCell<Monkey>], worry_update_fn: impl Fn(usize) -> usize) {
        for item in &self.items {
            self.inspections += 1;
            let new_worry_level = worry_update_fn(self.op.apply(*item));
            let throw_to = if new_worry_level % self.test.divisible_by == 0 {
                self.test.if_true
            } else {
                self.test.if_false
            };
            monkeys[throw_to].borrow_mut().items.push(new_worry_level);
        }
        self.items.clear();
    }
}

fn solve(input: &[Monkey], iterations: usize, worry_update_fn: impl Fn(usize) -> usize) -> usize {
    let mut input = input
        .iter()
        .map(|x| RefCell::new(x.clone()))
        .collect::<Vec<_>>();

    for _ in 0..iterations {
        for monkey in &input {
            monkey.borrow_mut().step(&input, &worry_update_fn);
        }
    }

    input.sort_by_key(|m| m.borrow().inspections);

    let last = input[input.len() - 1].borrow().inspections;
    let second_last = input[input.len() - 2].borrow().inspections;

    last * second_last
}

fn part1(input: &[Monkey]) -> usize {
    solve(input, 20, |x| x / 3)
}

fn part2(input: &[Monkey]) -> usize {
    let div_multiplier = input.iter().map(|m| m.test.divisible_by).product::<usize>();
    solve(input, 10_000, |x| x % div_multiplier)
}

fn parse(input: &str) -> Result<Vec<Monkey>, Box<dyn Error>> {
    Ok(monkey_parser::monkeys(input)?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    get_argv_reader().get_mut().read_to_string(&mut s)?;
    if !s.ends_with('\n') {
        s.push('\n');
    }
    let input = parse(&s)?;

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
                input_path: "inputs/11",
                part1_expected: 64032,
                part2_expected: 12729522272,
            },
            TestCase {
                input_path: "inputs/extra/11.sample",
                part1_expected: 10605,
                part2_expected: 2713310158,
            },
        ];

        for case in cases {
            let s = case.load_file();
            let input = parse(&s).unwrap();
            println!("{:?}", input);
            assert_eq!(part1(&input), case.part1_expected);
            assert_eq!(part2(&input), case.part2_expected);
        }
    }
}
