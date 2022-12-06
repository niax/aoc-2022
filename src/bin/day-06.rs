use aoc2022::commons::io::load_argv_lines;
use std::collections::HashSet;
use std::error::Error;

fn solve(input: &String, len: usize) -> usize {
    for i in 0..(input.len() - len) {
        let substr = &input[i..i + len];
        let charset = substr.chars().collect::<HashSet<char>>();
        if charset.len() == len {
            return i + len;
        }
    }
    panic!("NOT HERE");
}

fn part1(input: &String) -> usize {
    solve(input, 4)
}

fn part2(input: &String) -> usize {
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
        let cases = [TestCase {
            input_path: "inputs/extra/06.sample",
            part1_expected: 7,
            part2_expected: 19,
        }];

        for case in cases {
            let s = case.load_file();
            let input = s.lines().next().unwrap().to_string();
            assert_eq!(part1(&input), case.part1_expected);
            assert_eq!(part2(&input), case.part2_expected);
        }
    }
}
