use aoc2022::commons::io::load_argv_lines;
use std::collections::HashMap;
use std::collections::HashSet;

fn priority_for_char(ch: char) -> usize {
    let c = ch as usize;
    if (65..92).contains(&c) {
        c - 64 + 26
    } else if (97..123).contains(&c) {
        c - 96
    } else {
        panic!("Bad priority: {}", c);
    }
}

fn main() {
    let input: Vec<String> = load_argv_lines().map(|res| res.unwrap()).collect();

    let mut part1 = 0;
    let mut part2 = 0;
    let mut group_items = HashMap::new();
    for (i, backpack) in input.iter().enumerate() {
        let compartment_size = backpack.len() / 2;
        let c1_range = ..compartment_size;

        let mut c1 = HashSet::new();
        let mut c2 = HashSet::new();
        let mut combined = HashSet::new();
        for (j, ch) in backpack.chars().enumerate() {
            let priority = priority_for_char(ch);
            if c1_range.contains(&j) {
                c1.insert(priority);
            } else {
                c2.insert(priority);
            };
            combined.insert(priority);
        }

        for c in combined {
            let entry = group_items.entry(c).or_insert(0);
            *entry += 1;
        }

        let mut inter = c1.intersection(&c2);
        let common = inter.next().unwrap();
        assert_eq!(inter.next(), None);

        part1 += common;

        if i % 3 == 2 {
            for (priority, count) in group_items {
                if count == 3 {
                    part2 += priority;
                }
            }
            group_items = HashMap::new();
        }
    }

    println!("{}", part1);
    println!("{}", part2);
}
