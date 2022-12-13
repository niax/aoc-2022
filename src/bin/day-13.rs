use aoc2022::commons::io::load_argv_lines;
use std::{error::Error, str::FromStr};

peg::parser! {
    grammar signal_parser() for str {
        rule number() -> u8
            = n:$(['0'..='9']+) {? n.parse().or(Err("bad number")) }

        rule number_enum() -> Signal
            = n:number() { Signal::Number(n) }

        rule signal_list() -> Signal
            = "[" s:signal() ** "," "]"
            {
                Signal::List(s)
        }

        rule empty_signal() -> Signal
            = "" {
                Signal::Empty
            }

        pub rule signal() -> Signal
            = signal_list() / number_enum();

        pub rule signal_top() -> Signal
            = signal() / empty_signal();
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Signal {
    Number(u8),
    List(Vec<Signal>),
    Empty,
}

impl std::cmp::Ord for Signal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl std::cmp::PartialOrd for Signal {
    fn partial_cmp(&self, right: &Self) -> Option<std::cmp::Ordering> {
        match (self, right) {
            (Signal::Number(l), Signal::Number(r)) => match l.cmp(r) {
                std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
                std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
                std::cmp::Ordering::Equal => None,
            },
            (Signal::List(l), Signal::List(r)) => {
                let mut r_iter = r.iter();

                for l in l.iter() {
                    if let Some(r) = r_iter.next() {
                        if let Some(res) = l.partial_cmp(r) {
                            return Some(res);
                        }
                    } else {
                        return Some(std::cmp::Ordering::Greater); // Right is shorter than length
                    }
                }

                if r_iter.next().is_some() {
                    Some(std::cmp::Ordering::Less) // r still has values, so is longer and orders before l
                } else {
                    None // Same length, not conclusive
                }
            }
            (Signal::Number(l), Signal::List(_)) => {
                let l_list = vec![Signal::Number(*l)];
                Signal::List(l_list).partial_cmp(right)
            }
            (Signal::List(_), Signal::Number(r)) => {
                let r_list = vec![Signal::Number(*r)];
                self.partial_cmp(&Signal::List(r_list))
            }
            _ => panic!("Not handled {:?}", (self, right)),
        }
    }
}

impl FromStr for Signal {
    type Err = peg::error::ParseError<peg::str::LineCol>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        signal_parser::signal_top(s)
    }
}

fn part1(input: &[Signal]) -> usize {
    let mut answer = 0;
    let mut idx = 1;
    let mut iter = input.iter();
    while let Some(left) = iter.next() {
        let right = iter.next().unwrap();

        if left.partial_cmp(right).unwrap().is_lt() {
            answer += idx;
        }
        // Discard empty
        iter.next();
        idx += 1;
    }
    answer
}

fn part2(input: &[Signal]) -> usize {
    let mut input = input
        .iter()
        .filter(|x| **x != Signal::Empty)
        .collect::<Vec<_>>();
    let divider1 = Signal::List(vec![Signal::List(vec![Signal::Number(2)])]);
    let divider2 = Signal::List(vec![Signal::List(vec![Signal::Number(6)])]);
    input.push(&divider1);
    input.push(&divider2);

    input.sort();

    let first_idx = input.binary_search(&&divider1).expect("First index");
    let second_idx = input.binary_search(&&divider2).expect("Second index");

    (first_idx + 1) * (second_idx + 1)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_argv_lines().collect::<Result<Vec<_>, _>>()?;

    println!("{}", part1(&input));
    println!("{}", part2(&input));

    Ok(())
}
