use aoc2022::commons::io::load_argv_lines;
use petgraph::{graph::NodeIndex, Direction, Graph};
use std::error::Error;

#[derive(Debug, Clone)]
struct INode {
    name: String,
    size: usize,
    child_sizes: usize,
}

fn update_sizes(g: &mut Graph<INode, ()>, current: NodeIndex<u32>) -> usize {
    let children = g
        .neighbors_directed(current, Direction::Outgoing)
        .collect::<Vec<_>>();
    let mut child_sizes = 0;
    for child in children {
        child_sizes += update_sizes(g, child);
    }
    let current_inode = g.node_weight_mut(current).unwrap();
    child_sizes += current_inode.size;
    current_inode.child_sizes = child_sizes;

    child_sizes
}

fn parse(input: &[String]) -> Vec<INode> {
    let mut g = Graph::<INode, ()>::new();
    let root = g.add_node(INode {
        name: "".to_string(),
        size: 0,
        child_sizes: 0,
    });
    let mut current = root;
    for line in input {
        let parts = line.split(' ').collect::<Vec<&str>>();
        match parts[0] {
            "$" => {
                match parts[1] {
                    "cd" => {
                        current = match parts[2] {
                            "/" => root,
                            ".." => {
                                let mut parent_nodes =
                                    g.neighbors_directed(current, Direction::Incoming);
                                parent_nodes.next().unwrap()
                            }
                            a => {
                                // Check for existing neighbour
                                let mut found_child = None;
                                for child in g.neighbors_directed(current, Direction::Outgoing) {
                                    if g.node_weight(child).unwrap().name == a {
                                        found_child = Some(child);
                                    }
                                }
                                if let Some(child) = found_child {
                                    child
                                } else {
                                    let new_node = g.add_node(INode {
                                        name: a.to_string(),
                                        size: 0,
                                        child_sizes: 0,
                                    });
                                    g.add_edge(current, new_node, ());

                                    new_node
                                }
                            }
                        };
                    }
                    "ls" => {}
                    _ => panic!("Dunno about {}", parts[1]),
                }
            }
            "dir" => {}
            size_str => {
                let new_node = g.add_node(INode {
                    name: parts[1].to_string(),
                    size: size_str.parse().unwrap(),
                    child_sizes: 0,
                });
                g.add_edge(current, new_node, ());
            }
        }
    }

    update_sizes(&mut g, root);
    let mut dir_sizes = g
        .node_indices()
        .map(|idx| g.node_weight(idx).unwrap())
        .cloned()
        .filter(|inode| inode.size == 0)
        .collect::<Vec<_>>();
    dir_sizes.sort_by_key(|inode| inode.child_sizes);
    dir_sizes
}

fn part1(input: &[INode]) -> usize {
    let mut sum = 0;
    for dir in input {
        if dir.child_sizes > 100000 {
            continue;
        }
        sum += dir.child_sizes;
    }

    sum
}

fn part2(input: &[INode]) -> usize {
    let available = 70000000 - input.last().unwrap().child_sizes;
    let want = 30000000;
    for dir in input {
        if (available + dir.child_sizes) > want {
            return dir.child_sizes;
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
