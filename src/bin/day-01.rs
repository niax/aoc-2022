use std::collections::BinaryHeap;

use aoc2022::commons::io::load_argv_records;

fn solve(sums: &mut BinaryHeap<u32>) -> (u32, u32) {
    let mut part1 = 0;
    let mut part2 = 0;
    for i in 0..3 {
        let n = match sums.pop() {
            Some(x) => x,
            None => break,
        };
        if i == 0 {
            part1 = n;
        }
        part2 += n;
    }

    (part1, part2)
}

fn main() {
    let mut sums: BinaryHeap<u32> = load_argv_records("")
        .map(|res| res.unwrap().iter().sum())
        .collect();

    let (part1, part2) = solve(&mut sums);
    println!("{}", part1);
    println!("{}", part2);
}

#[cfg(test)]
mod tests {
    use aoc2022::commons::test_helpers::TestCase;
    use aoc2022::commons::io::parse_records;

    use super::*;


    #[test]
    fn test_solution() {
        let cases = [
            TestCase {
                input_path: "inputs/01",
                part1_expected: 71780,
                part2_expected: 212489,
            },
            TestCase {
                input_path: "inputs/extra/01.sample",
                part1_expected: 24000,
                part2_expected: 45000,
            },
        ];

        for case in cases {
            let s = case.load_file();
            let mut input = parse_records(s.lines().map(|l| Ok(l.to_string())), "".to_string())
                .map(|res| res.unwrap().iter().sum())
                .collect();
            let (part1, part2) = solve(&mut input);
            assert_eq!(part1, case.part1_expected);
            assert_eq!(part2, case.part2_expected);
        }
    }
}
