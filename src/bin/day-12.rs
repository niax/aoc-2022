use aoc2022::commons::io::load_argv_lines;
use petgraph::{algo::astar, graph::NodeIndex, Graph};
use std::{collections::HashMap, error::Error};

fn part1(g: &Graph<isize, usize>, start: NodeIndex<u32>, end: NodeIndex<u32>) -> usize {
    let (cost, _) = astar(
        &g,
        end,
        |finish| finish == start,
        |e| *e.weight(),
        |_| 0,
    )
    .expect("Should be a path");
    cost
}

fn part2(g: &Graph<isize, usize>, end: NodeIndex<u32>) -> usize {
    let (cost, _) = astar(
        &g,
        end,
        |finish| g.node_weight(finish).unwrap() == &1,
        |e| *e.weight(),
        |_| 0,
    )
    .unwrap();
    cost
}

fn build_graph(input: &[String]) -> (Graph<isize, usize>, NodeIndex<u32>, NodeIndex<u32>) {
    let mut g = Graph::new();
    let mut node_idx = HashMap::with_capacity(1024);
    let mut start_node = None;
    let mut end_node = None;
    for (y, l) in input.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let height = match c {
                'S' => 0,
                'E' => 26,
                _ => c as isize - 96,
            };
            let idx = g.add_node(height);
            node_idx.insert((x as isize, y as isize), idx);
            

            if c == 'S' {
                start_node = Some(idx);
            } else if c == 'E' {
                end_node = Some(idx);
            }
        }
    }

    for ((x, y), idx) in node_idx.iter() {
        let height = *g.node_weight(*idx).unwrap();
        for (x_delta, y_delta) in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
            let adj_x = *x + x_delta;
            let adj_y = *y + y_delta;

            if let Some(&adj_idx) = node_idx.get(&(adj_x, adj_y)) {
                let adj_height = g.node_weight(adj_idx).expect("Find adjacent node");

                if *adj_height - height <= 1 {
                    g.add_edge(adj_idx, *idx, 1);
                }
            }
        }
    }

    (g, start_node.expect("Find start node"), end_node.expect("Find end node"))
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_argv_lines().collect::<Result<Vec<_>, _>>()?;

    let (g, start, end) = build_graph(&input);

    println!("{}", part1(&g, start, end));
    println!("{}", part2(&g, end));

    Ok(())
}

#[cfg(test)]
mod tests {
    use aoc2022::commons::test_helpers::TestCase;

    use super::*;

    #[test]
    fn test_solution() {
        let cases = [
            TestCase {
                input_path: "inputs/12",
                part1_expected: 423,
                part2_expected: 416,
            },
            TestCase {
                input_path: "inputs/extra/12.sample",
                part1_expected: 31,
                part2_expected: 29,
            },
        ];

        for case in cases {
            let input = case
                .load_file_lines()
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            let (g, start, end) = build_graph(&input);
            assert_eq!(part1(&g, start, end), case.part1_expected);
            assert_eq!(part2(&g, end), case.part2_expected);
        }
    }
}
