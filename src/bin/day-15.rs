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

fn part2(input: &[Reading]) -> usize {
    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_argv_lines().collect::<Result<Vec<String>, _>>()?;
    let input = input.iter().map(|i| sensor::reading(&i)).collect::<Result<Vec<_>, _>>()?;

    println!("{}", part1(&input));
    println!("{}", part2(&input));

    Ok(())
}
