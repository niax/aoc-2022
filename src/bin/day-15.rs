use aoc2022::commons::{geom::Point, io::load_argv_lines};
use std::{collections::HashSet, error::Error};

#[derive(Debug)]
pub struct Reading {
    sensor: Point<isize>,
    beacon: Point<isize>,
}

peg::parser! {
    grammar sensor() for str {
        rule number() -> isize
            = n:$("-"? ['0'..='9']+) {? n.parse().or(Err("bad number")) }

        rule point() -> Point<isize>
            = "x=" x:number() ", y=" y:number() {
                Point::new(x, y)
            }

        pub rule reading() -> Reading
            = "Sensor at " sensor:point() ": closest beacon is at " beacon:point() {
                Reading{sensor, beacon}
            }
    }
}

struct Ranges {
    ranges: Vec<(isize, isize)>,
}

impl Ranges {
    pub fn new() -> Self {
        Self {
            ranges: Vec::with_capacity(1_000),
        }
    }

    pub fn add(&mut self, range: (isize, isize)) {
        self.ranges.push(range);
    }

    pub fn compat(&mut self) {
        self.ranges.sort();
        let mut new_ranges = Vec::with_capacity(self.ranges.len());
        let mut current_min = self.ranges[0].0;
        let mut current_max = self.ranges[0].1;
        for range in &self.ranges {
            let (min, max) = range;
            if min - 1 <= current_max {
                // This range extends/contains
                current_min = current_min.min(*min);
                current_max = current_max.max(*max);
            } else {
                // This is new, and doesn't extend current
                new_ranges.push((current_min, current_max));
                current_min = *min;
                current_max = *max;
            }
        }
        new_ranges.push((current_min, current_max));
        self.ranges = new_ranges;
    }
}

fn ranges_at_row(input: &[Reading], y: isize) -> Ranges {
    let mut ranges = Ranges::new();
    for reading in input {
        let max_distance = reading.sensor.x().abs_diff(*reading.beacon.x())
            + reading.sensor.y().abs_diff(*reading.beacon.y());

        let distance = reading.sensor.y().abs_diff(y as isize);
        if distance <= max_distance {
            let max_x_delta = distance.abs_diff(max_distance);
            ranges.add((
                reading.sensor.x() - max_x_delta as isize,
                reading.sensor.x() + max_x_delta as isize,
            ))
        }
    }
    ranges.compat();
    ranges
}

const PART1_Y: isize = 2000000;

fn part1(input: &[Reading]) -> usize {
    let becons_on_row = input
        .iter()
        .filter_map(|x| {
            if *x.beacon.y() == PART1_Y {
                Some(x.beacon.tuple_copy())
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();
    let ranges = ranges_at_row(input, PART1_Y);

    ranges
        .ranges
        .iter()
        .map(|x| x.0.abs_diff(x.1))
        .sum::<usize>()
        - becons_on_row.len()
        + 1
}

const PART2_MAX: usize = 4_000_000;
//const PART2_MAX: usize = 20;

fn part2(input: &[Reading]) -> usize {
    for y in 0..=PART2_MAX {
        let mut ranges = Ranges::new();
        for reading in input {
            let max_distance = reading.sensor.x().abs_diff(*reading.beacon.x())
                + reading.sensor.y().abs_diff(*reading.beacon.y());

            let distance = reading.sensor.y().abs_diff(y as isize);
            if distance <= max_distance {
                let max_x_delta = distance.abs_diff(max_distance);
                ranges.add((
                    reading.sensor.x() - max_x_delta as isize,
                    reading.sensor.x() + max_x_delta as isize,
                ))
            }
        }
        ranges.compat();
        if ranges.ranges.len() != 1 {
            return ((ranges.ranges[0].1 as usize + 1) * 4000000) + y;
        }
    }

    unreachable!();
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_argv_lines().collect::<Result<Vec<String>, _>>()?;
    let input = input
        .iter()
        .map(|i| sensor::reading(i))
        .collect::<Result<Vec<_>, _>>()?;

    println!("{}", part1(&input));
    println!("{}", part2(&input));

    Ok(())
}
