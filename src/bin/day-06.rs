use aoc2022::commons::io::load_argv_lines;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::hash::Hash;

#[derive(Debug)]
struct Window<T> {
    counts: HashMap<T, usize>,
    window: VecDeque<Option<T>>,
}

impl<T: Eq + Hash + Copy> Window<T> {
    fn new(len: usize) -> Window<T> {
        let mut window = VecDeque::with_capacity(len);
        for _ in 0..len {
            window.push_back(None);
        }
        Window {
            counts: HashMap::new(),
            window,
        }
    }

    fn push(&mut self, i: T) {
        let entry = self.counts.entry(i);
        let previous = entry.or_insert(0);
        *previous += 1;
        self.window.push_back(Some(i));

        let fall_off = self
            .window
            .pop_front()
            .expect("Window should never be empty");
        if let Some(j) = fall_off {
            let falling_off_count = self
                .counts
                .get_mut(&j)
                .expect("Previous count should not be empty");
            *falling_off_count -= 1;
            if *falling_off_count == 0 {
                self.counts.remove(&j);
            }
        }
    }

    fn unique_count(&self) -> usize {
        self.counts.len()
    }
}

fn solve(input: &str, len: usize) -> usize {
    let mut window = Window::new(len);

    for (i, c) in input.chars().into_iter().enumerate() {
        window.push(c);
        if window.unique_count() == len {
            return i + 1;
        }
    }
    panic!("NOT HERE");
}

fn part1(input: &str) -> usize {
    solve(input, 4)
}

fn part2(input: &str) -> usize {
    solve(input, 14)
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = load_argv_lines().collect::<Result<Vec<String>, _>>()?;
    let input = &lines[0];

    println!("{}", part1(input));
    println!("{}", part2(input));

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
                input_path: "inputs/extra/06.sample",
                part1_expected: 7,
                part2_expected: 19,
            },
            TestCase {
                input_path: "inputs/06",
                part1_expected: 1760,
                part2_expected: 2974,
            },
        ];

        for case in cases {
            let s = case.load_file();
            let input = s.lines().next().unwrap().to_string();
            assert_eq!(part1(&input), case.part1_expected);
            assert_eq!(part2(&input), case.part2_expected);
        }
    }
}
