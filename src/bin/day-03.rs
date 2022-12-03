use aoc2022::commons::io::load_argv_lines;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input: Vec<String> = load_argv_lines().map(|res| res.unwrap()).collect();

    let mut part1 = 0;
    let mut part2 = 0;
    let mut group_items = HashMap::new();
    for (i, backpack) in input.iter().enumerate() {
        let compartment_size = backpack.len() / 2;
        let c1: HashSet<char> = backpack[..compartment_size].chars().collect();
        let c2: HashSet<char> = backpack[compartment_size..].chars().collect();

        let combined: HashSet<char> = backpack.chars().collect();
        for c in combined {
            let entry = group_items.entry(c).or_insert(0);
            *entry += 1;
        }

        let mut inter = c1.intersection(&c2);
        let common = inter.next().unwrap();
        assert_eq!(inter.next(), None);

        let charcode = *common as u32;

        let priority = if (65..92).contains(&charcode) {
            charcode - 64 + 26
        } else if (97..123).contains(&charcode) {
            charcode - 96
        } else {
            panic!("Bad priority");
        };
        part1 += priority;

        if i % 3 == 2 {
            for (item, count) in group_items {
                let charcode = item as u32;
                if count == 3 {
                    let priority = if (65..92).contains(&charcode) {
                        charcode - 64 + 26
                    } else if (97..123).contains(&charcode) {
                        charcode - 96
                    } else {
                        panic!("Bad priority");
                    };
                    part2 += priority;
                }
            }
            group_items = HashMap::new();
        }
    }

    println!("{}", part1);
    println!("{}", part2);
}
