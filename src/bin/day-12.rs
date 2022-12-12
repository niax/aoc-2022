use aoc2022::commons::io::load_argv_lines;
use petgraph::{algo::astar, graph::NodeIndex, Graph};
use std::{collections::HashMap, error::Error};

fn part1(input: &[String]) -> usize {
    let (g, chars) = build_graph(input, true);
    let start_node = chars.get(&'S').unwrap()[0];
    let end_node = chars.get(&'E').unwrap()[0];

    let (cost, _) = astar(
        &g,
        start_node,
        |finish| finish == end_node,
        |e| *e.weight(),
        |_| 0,
    )
    .expect("Should be a path");
    cost
}

fn part2(input: &[String]) -> usize {
    let (g, chars) = build_graph(input, false);

    let end_node = chars.get(&'E').unwrap()[0];

    let (cost, _) = astar(
        &g,
        end_node,
        |finish| g.node_weight(finish).unwrap() == &1,
        |e| *e.weight(),
        |_| 0,
    )
    .unwrap();
    cost
}

type CharIdxMap = HashMap<char, Vec<NodeIndex<u32>>>;

fn build_graph(input: &[String], part_1_order: bool) -> (Graph<isize, usize>, CharIdxMap) {
    let mut g = Graph::new();
    let mut node_idx = HashMap::new();
    let mut char_places = HashMap::new();
    for (y, l) in input.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let height = match c {
                'S' => 0,
                'E' => 26,
                _ => c as isize - 96,
            };
            let idx = g.add_node(height);
            node_idx.insert((x as isize, y as isize), idx);

            let entry = char_places.entry(c);
            let l = entry.or_insert_with(Vec::new);
            l.push(idx);
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
                    if part_1_order {
                        g.add_edge(*idx, adj_idx, 1);
                    } else {
                        g.add_edge(adj_idx, *idx, 1);
                    }
                }
            }
        }
    }

    (g, char_places)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_argv_lines().collect::<Result<Vec<_>, _>>()?;

    println!("{}", part1(&input));
    println!("{}", part2(&input));

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
            assert_eq!(part1(&input), case.part1_expected);
            assert_eq!(part2(&input), case.part2_expected);
        }
    }
}
