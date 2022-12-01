use aoc2022::commons::io::load_argv_records;

fn main() {
    let ints: Vec<Vec<u32>> = load_argv_records("").map(|res| res.unwrap()).collect();
    let mut sums: Vec<u32> = ints.iter().map(|v| v.iter().sum()).collect();
    sums.sort_unstable_by(|a, b| b.cmp(a));

    println!("{:?}", sums[0]);
    println!("{:?}", sums[0..3].iter().sum::<u32>());
}
