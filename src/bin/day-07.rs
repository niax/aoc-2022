use aoc2022::commons::io::get_argv_reader;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug)]
pub enum LsEntry {
    Directory(String),
    File(usize, String),
}

#[derive(Debug)]
pub enum Command {
    Ls(Vec<LsEntry>),
    Cd(String),
}

peg::parser! {
    grammar shell_parser() for str {
        rule number() -> usize
            = n:$(['0'..='9']+) {? n.parse().or(Err("bad number")) }

        rule path() -> String
            = n:$(['!'..='~']+) { n.to_string() }

        rule dir_entry() -> LsEntry
            = "dir " p:path() { LsEntry::Directory(p.to_string()) }

        rule file_entry() -> LsEntry
            = size:number() " " p:path() { LsEntry::File(size, p.to_string()) }

        rule ls_entry() -> LsEntry
            = (dir_entry() / file_entry())

        rule ls_line() -> LsEntry
            = entry:ls_entry() "\n" { entry }

        rule ls_lines() -> Vec<LsEntry>
            = ls_line()+

        rule ls() -> Command
            = "$ ls\n" lines:ls_lines() {
                Command::Ls(lines)
        }

        rule cd() -> Command
            = "$ cd " p:path() "\n" {
                Command::Cd(p.to_string())
        }

        rule cmd_entry() -> Command
            = ls() / cd()

        pub rule cmds() -> Vec<Command>
            = cmd_entry()+
    }
}

fn parse(input: &str) -> Result<Vec<usize>, Box<dyn Error>> {
    let cmds = shell_parser::cmds(input)?;

    let mut sizes = HashMap::new();
    let root = PathBuf::new();
    let mut current = root.clone();

    for cmd in cmds {
        match cmd {
            Command::Cd(path) => {
                current = match path.as_str() {
                    "/" => root.to_path_buf(),
                    ".." => current.parent().unwrap().to_path_buf(),
                    _a => current.join(path),
                };
            }
            Command::Ls(files) => {
                for file in files {
                    match file {
                        LsEntry::File(size, _) => {
                            let c = current.clone();
                            for ancestor in c.ancestors() {
                                let entry = sizes.entry(ancestor.to_path_buf());
                                let ancestor_size = entry.or_insert(0);
                                *ancestor_size += size;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    let mut dir_sizes = sizes.values().copied().collect::<Vec<_>>();
    dir_sizes.sort();
    Ok(dir_sizes)
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
            let input = parse(&s).unwrap();
            assert_eq!(part1(&input), case.part1_expected);
            assert_eq!(part2(&input), case.part2_expected);
        }
    }
}
