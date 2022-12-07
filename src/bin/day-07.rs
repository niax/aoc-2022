use aoc2022::commons::io::load_argv_lines;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

fn parse(input: &[String]) -> Vec<usize> {
    let mut sizes = HashMap::new();
    let root = PathBuf::new();
    let mut current = root.clone();

    for line in input {
        let parts = line.split(' ').collect::<Vec<&str>>();
        match parts[0] {
            "$" => match parts[1] {
                "cd" => {
                    current = match parts[2] {
                        "/" => root.to_path_buf(),
                        ".." => current.parent().unwrap().to_path_buf(),
                        a => current.join(a),
                    };
                }
                "ls" => {}
                _ => panic!("Dunno about {}", parts[1]),
            },
            "dir" => {}
            size_str => {
                let size = size_str.parse::<usize>().unwrap();
                let c = current.clone();
                for ancestor in c.ancestors() {
                    let entry = sizes.entry(ancestor.to_path_buf());
                    let ancestor_size = entry.or_insert(0);
                    *ancestor_size += size;
                }
            }
        }
    }

    let mut dir_sizes = sizes.values().copied().collect::<Vec<_>>();
    dir_sizes.sort();
    dir_sizes
}

fn part1(input: &[usize]) -> usize {
    let mut sum = 0;
    for &size in input {
        if size > 100000 {
            continue;
        }
        sum += size;
    }

    sum
}

fn part2(input: &[usize]) -> usize {
    let available = 70000000 - input.last().unwrap();
    let want = 30000000;
    for &size in input {
        if (available + size) > want {
            return size;
        }
    }
    panic!("not found");
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = parse(&load_argv_lines().collect::<Result<Vec<String>, _>>()?);

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
                input_path: "inputs/extra/07.sample",
                part1_expected: 95437,
                part2_expected: 24933642,
            },
            TestCase {
                input_path: "inputs/07",
                part1_expected: 1644735,
                part2_expected: 1300850,
            },
        ];

        for case in cases {
            let s = case.load_file();
            let input = parse(
                &s.lines()
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<String>>(),
            );
            assert_eq!(part1(&input), case.part1_expected);
            assert_eq!(part2(&input), case.part2_expected);
        }
    }
}
