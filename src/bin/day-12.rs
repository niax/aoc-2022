use aoc2022::commons::{
    grid::{Grid, SingleVecGrid},
    io::load_argv_lines,
};
use petgraph::{algo::dijkstra, Graph};
use std::{collections::HashMap, error::Error};

fn solve(input: &[String]) -> (isize, isize) {
    let mut g = Graph::new();
    let mut node_grid = SingleVecGrid::new(input[0].len(), input.len());
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
            node_grid.set((x, y), idx);

            if c == 'S' {
                start_node = Some(idx);
            } else if c == 'E' {
                end_node = Some(idx);
            }
        }
    }

    for y in 0..node_grid.height() {
        for x in 0..node_grid.width() {
            let idx = node_grid.at(&(x, y)).unwrap();
            let height = *g.node_weight(*idx).unwrap();
            for (_, &adj_idx) in node_grid.adjacent((x, y)) {
                let adj_height = g.node_weight(adj_idx).expect("Find adjacent node");

                if *adj_height - height <= 1 {
                    g.add_edge(adj_idx, *idx, 1);
                }
            }
        }
    }

    let costs = dijkstra(&g, end_node.unwrap(), None, |e| *e.weight());
    let mut weight_costs = HashMap::with_capacity(1024);
    for (node_idx, cost) in &costs {
        let weight = g.node_weight(*node_idx).unwrap();
        let entry = weight_costs.entry(weight);
        let current = entry.or_insert(isize::MAX);
        if *current > *cost {
            *current = *cost;
        }
    }

    (
        *costs.get(&start_node.unwrap()).unwrap(),
        *weight_costs.get(&1).unwrap(),
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_argv_lines().collect::<Result<Vec<_>, _>>()?;

    let (part1, part2) = solve(&input);

    println!("{}", part1);
    println!("{}", part2);

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
            let (part1, part2) = solve(&input);
            assert_eq!(part1, case.part1_expected);
            assert_eq!(part2, case.part2_expected);
        }
    }
}
