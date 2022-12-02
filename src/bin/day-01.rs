use std::collections::BinaryHeap;

use aoc2022::commons::io::load_argv_records;

fn main() {
    let mut sums: BinaryHeap<u32> = load_argv_records("")
        .map(|res| res.unwrap().iter().sum())
        .collect();

    let mut sum = 0;
    for i in 0..3 {
        let n = match sums.pop() {
            Some(x) => x,
            None => break,
        };
        if i == 0 {
            // Part 1
            println!("{}", n);
        }
        sum += n;
    }

    // Part 2
    println!("{}", sum);
}
