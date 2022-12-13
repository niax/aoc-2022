use aoc2022::commons::io::load_argv_lines;
use std::{error::Error, str::FromStr};

peg::parser!{
    grammar signal_parser() for str {
        rule number() -> usize
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

#[derive(Debug)]
pub enum Signal {
    Number(usize),
    List(Vec<Signal>),
    Empty,
}

impl Signal {
    pub fn orders_before(&self, right: &Signal) -> Option<bool> {
        println!("Compare {:?} with {:?}", self, right);
        match (self, right) {
            (Signal::Number(l), Signal::Number(r)) => {
                match l.cmp(r) {
                    std::cmp::Ordering::Less => Some(true),
                    std::cmp::Ordering::Greater => Some(false),
                    std::cmp::Ordering::Equal => None,
                }
            },
            (Signal::List(l), Signal::List(r)) => {
                let mut l_iter = l.iter();
                let mut r_iter = r.iter();

                while let Some(l) = l_iter.next() {
                    if let Some(r) = r_iter.next() {
                        if let Some(res) = l.orders_before(r) {
                            return Some(res);
                        }
                    } else {
                        return Some(false); // Right is shorter than length
                    }
                }

                if r_iter.next().is_some() {
                    Some(true) // r still has values, so is longer and orders before l
                } else {
                    None
                }
            }
            (Signal::Number(l), Signal::List(_)) => {
                let mut l_list = Vec::with_capacity(1);
                l_list.push(Signal::Number(*l));
                Signal::List(l_list).orders_before(right)
            },
            (Signal::List(_), Signal::Number(r)) => {
                let mut r_list = Vec::with_capacity(1);
                r_list.push(Signal::Number(*r));
                self.orders_before(&Signal::List(r_list))
            },
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

        if left.orders_before(&right).unwrap() {
            answer += idx;
        }
        // Discard empty
        iter.next();
        idx += 1;
    }
    answer
}

fn part2(input: &[Signal]) -> usize {
    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_argv_lines().collect::<Result<Vec<_>, _>>()?;

    println!("{}", part1(&input));
    println!("{}", part2(&input));

    Ok(())
}
