use aoc2022::commons::io::load_argv_records;

fn main() {
    let ints: Vec<Vec<u32>> = load_argv_records("").map(|res| res.unwrap()).collect();
    let mut sums: Vec<u32> = ints.iter().map(|v| v.iter().sum()).collect();
    sums.sort_unstable();

    println!("{:?}", sums.iter().last().unwrap());
    println!("{:?}", sums.iter().rev().take(3).sum::<u32>());
}
