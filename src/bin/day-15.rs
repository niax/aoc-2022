use aoc2022::commons::{
    io::load_argv_lines,
    grid::{SparseGrid, Grid},
    geom::Point,
};
use std::{error::Error, collections::HashSet};

#[derive(Debug)]
pub struct Reading {
    sensor: Point<isize>,
    beacon: Point<isize>,
}

peg::parser!{
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

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Sensor,
    Beacon,
}

fn part1(input: &[Reading]) -> usize {
    let mut grid = SparseGrid::new();

    let mut seen_tiles = HashSet::with_capacity(10000);
    let wanted_y = 2000000;

    for reading in input {
        grid.set(reading.beacon.tuple_copy(), Tile::Beacon);
        grid.set(reading.sensor.tuple_copy(), Tile::Sensor);

        let max_distance = reading.sensor.x().abs_diff(*reading.beacon.x()) +
            reading.sensor.y().abs_diff(*reading.beacon.y());

        for dx in [-1, 1] {
            let mut point = Point::new(*reading.sensor.x(), wanted_y);
            let mut distance = reading.sensor.y().abs_diff(*point.y());
            while distance <= max_distance {
                //println!("{:?} - {} ({})", point, distance, max_distance);
                let t = point.tuple_copy();
                if grid.at(&t).is_none() {
                    seen_tiles.insert(t);
                }
                point += (dx, 0);
                distance += 1;

            }
        }
    }

    seen_tiles.len()
}

struct Ranges {
    ranges: Vec<(isize, isize)>
}

impl Ranges {
    pub fn new() -> Self {
        Self {
            ranges: Vec::with_capacity(1_000),
        }
    }

    pub fn add(&mut self, range: (isize, isize)) {
        let new_range = (range.0.max(0), range.1.min(PART2_MAX as isize));
        self.ranges.push(new_range);
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

const PART2_MAX: usize = 4_000_000;
//const PART2_MAX: usize = 20;

fn part2(input: &[Reading]) -> usize {

    for y in 0..=PART2_MAX {
        let mut ranges = Ranges::new();
        for reading in input {
            let max_distance = reading.sensor.x().abs_diff(*reading.beacon.x()) +
                reading.sensor.y().abs_diff(*reading.beacon.y());

            let distance = reading.sensor.y().abs_diff(y as isize);
            if distance <= max_distance {
                let max_x_delta = distance.abs_diff(max_distance);
                ranges.add((reading.sensor.x() - max_x_delta as isize, reading.sensor.x() + max_x_delta as isize))
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
    let input = input.iter().map(|i| sensor::reading(&i)).collect::<Result<Vec<_>, _>>()?;

    println!("{}", part1(&input));
    println!("{}", part2(&input));

    Ok(())
}
