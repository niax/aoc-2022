use aoc2022::commons::io::load_argv_lines;
use itertools::Itertools;
use petgraph::algo::floyd_warshall;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::error::Error;

peg::parser! {
    grammar valve_parser() for str {
        rule number() -> usize
            = n:$(['0'..='9']+) {? n.parse().or(Err("bad number")) }

        rule valve() -> String
            = n:$(['A'..='Z']+) { n.to_string() }

        rule valve_list() -> Vec<String>
            = n:valve() ** ", " { n }

        pub rule valve_line() -> Valve
            = "Valve " v:valve() " has flow rate=" r:number() "; tunnel" "s"? " lead" "s"? " to valve" ("s "/" ") l:valve_list() {
                Valve{
                    name: v,
                    flow_rate: r,
                    connected_to: l}
            }
    }
}

#[derive(Debug)]
pub struct Valve {
    name: String,
    flow_rate: usize,
    connected_to: Vec<String>,
}

fn build_graph(valves: &[Valve]) -> Graph<&Valve, ()> {
    let mut valve_map = HashMap::new();
    let mut g = Graph::new();
    for valve in valves {
        let node = g.add_node(valve);
        valve_map.insert(valve.name.clone(), node);
    }

    for valve in valves {
        let src = valve_map.get(&valve.name).unwrap();
        for dst_name in &g.node_weight(*src).unwrap().connected_to {
            let dst = valve_map.get(dst_name).unwrap();
            g.add_edge(*src, *dst, ());
        }
    }

    g
}

#[derive(Debug)]
#[allow(dead_code)]
struct PathElem<'a> {
    cost: usize,
    flow: usize,
    name: &'a str,
}

fn max_flow<'a>(
    g: &Graph<&'a Valve, ()>,
    from: NodeIndex<u32>,
    costs: &HashMap<NodeIndex<u32>, HashMap<NodeIndex<u32>, usize>>,
    visited: &HashSet<NodeIndex<u32>>,
    remaining_steps: usize,
) -> (usize, Vec<PathElem<'a>>) {
    let from_flow = g.node_weight(from).unwrap().flow_rate * remaining_steps;
    let mut visited = visited.clone();
    visited.insert(from);

    let mut max_flow_rate = 0;
    let mut path = Vec::with_capacity(g.capacity().0);
    for (neighbour, cost) in costs.get(&from).unwrap() {
        if !visited.contains(neighbour) {
            let steps_after_neighbour = remaining_steps.checked_sub(*cost + 1);
            if let Some(remaining_steps_after_neighbour) = steps_after_neighbour {
                let (max_flow, path_elem) = max_flow(
                    g,
                    *neighbour,
                    costs,
                    &visited,
                    remaining_steps_after_neighbour,
                );
                if max_flow > max_flow_rate {
                    let to_valve = g.node_weight(*neighbour).unwrap();
                    max_flow_rate = max_flow;
                    path.clear();
                    path.push(PathElem {
                        cost: *cost,
                        flow: g.node_weight(*neighbour).unwrap().flow_rate,
                        name: &to_valve.name,
                    });
                    path.extend(path_elem)
                }
            }
        }
    }

    (max_flow_rate + from_flow, path)
}

fn part1(g: &Graph<&Valve, ()>) -> usize {
    let all_pairs = floyd_warshall(g, |_| 1).expect("Couldn't figure out paths");
    let mut worthwhile_valve_costs = HashMap::new();
    for ((from, to), v) in all_pairs {
        let from_valve = g.node_weight(from).expect("valve node");
        if from_valve.flow_rate == 0 && from_valve.name != "AA" {
            continue;
        }
        let to_valve = g.node_weight(to).expect("valve node");
        if to_valve.flow_rate == 0 && to_valve.name != "AA" {
            continue;
        }
        let e = worthwhile_valve_costs.entry(from);
        let map: &mut HashMap<NodeIndex<u32>, usize> = e.or_default();
        map.insert(to, v);
    }

    let start = g
        .node_indices()
        .find(|idx| g.node_weight(*idx).unwrap().name == "AA")
        .unwrap();

    let (flow, _) = max_flow(g, start, &worthwhile_valve_costs, &HashSet::new(), 30);
    flow
}

#[derive(Debug)]
struct FlowState {
    at: NodeIndex<u32>,
    opened_valves: BTreeSet<NodeIndex<u32>>,
    remaining_minutes: usize,
    flow_at_end: usize,
}

fn part2(g: &Graph<&Valve, ()>) -> usize {
    let all_pairs = floyd_warshall(g, |_| 1).expect("Couldn't figure out paths");
    let mut worthwhile_valve_costs = HashMap::new();
    for ((from, to), v) in all_pairs {
        let from_valve = g.node_weight(from).expect("valve node");
        if from_valve.flow_rate == 0 && from_valve.name != "AA" {
            continue;
        }
        let to_valve = g.node_weight(to).expect("valve node");
        if to_valve.flow_rate == 0 {
            continue;
        }
        let e = worthwhile_valve_costs.entry(from);
        let map: &mut HashMap<NodeIndex<u32>, usize> = e.or_default();
        map.insert(to, v);
    }

    let start = g
        .node_indices()
        .find(|idx| g.node_weight(*idx).unwrap().name == "AA")
        .unwrap();

    let mut best_flow_for_valves_open: HashMap<BTreeSet<NodeIndex>, usize> = HashMap::new();
    let mut queue = VecDeque::with_capacity(1024);
    queue.push_back(FlowState {
        at: start,
        opened_valves: BTreeSet::new(),
        remaining_minutes: 26,
        flow_at_end: 0,
    });

    while let Some(state) = queue.pop_front() {
        for (neighbour, cost) in worthwhile_valve_costs.get(&state.at).unwrap() {
            if state.opened_valves.contains(neighbour) {
                continue;
            }

            let steps_after_neighbour = state.remaining_minutes.checked_sub(*cost + 1);
            if let Some(remaining_steps_after_neighbour) = steps_after_neighbour {
                // We have time left to visit this valve
                let flow_at_end_from_valve =
                    g.node_weight(*neighbour).unwrap().flow_rate * remaining_steps_after_neighbour;
                let combined_flow_at_end = state.flow_at_end + flow_at_end_from_valve;
                let mut new_visited = state.opened_valves.clone();
                new_visited.insert(*neighbour);

                best_flow_for_valves_open
                    .entry(new_visited.clone())
                    .and_modify(|v| *v = combined_flow_at_end.max(*v))
                    .or_insert(combined_flow_at_end);

                queue.push_back(FlowState {
                    at: *neighbour,
                    opened_valves: new_visited,
                    flow_at_end: combined_flow_at_end,
                    remaining_minutes: remaining_steps_after_neighbour,
                })
            }
        }
    }

    best_flow_for_valves_open
        .iter()
        .tuple_combinations()
        .filter(|((p1, _), (p2, _))| p1.is_disjoint(p2))
        .map(|((_, p1), (_, p2))| p1 + p2)
        .max()
        .unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<String> = load_argv_lines().collect::<Result<_, _>>()?;
    let input = input
        .iter()
        .map(|s| valve_parser::valve_line(s))
        .collect::<Result<Vec<_>, _>>()?;
    let g = build_graph(&input);

    println!("{}", part1(&g));
    println!("{}", part2(&g));

    Ok(())
}
