use aoc2022::commons::{
    grid::{Grid, SparseGrid},
    io::load_argv_lines,
};
use std::error::Error;

peg::parser! {
    grammar probe_reports() for str {
        rule number() -> isize
            = n:$(['0'..='9']+) {? n.parse().or(Err("bad number")) }

        rule point() -> (isize, isize)
            = l:number() "," r:number() {
                (l, r)
            }

        pub rule path() -> Vec<(isize, isize)>
            = point() ** " -> "
    }
}

fn part1(input: &[Vec<(isize, isize)>]) -> isize {
    let mut grid = SparseGrid::new();
    let mut bottom_point = 0;

    for path in input {
        let mut path_iter = path.iter();
        let mut last = path_iter
            .next()
            .expect("path should be at least 1 long")
            .clone();
        for n in path_iter {
            let dir = ((n.0 - last.0).signum(), (n.1 - last.1).signum());
            grid.set(*n, true);

            while last != *n {
                grid.set(last, true);
                bottom_point = bottom_point.max(last.1);
                last = ((last.0 + dir.0), (last.1 + dir.1));
            }
        }
    }

    let sand_drop = (500, 0);
    let mut placed = 0;

    'outer: loop {
        let mut sand_pos = sand_drop.clone();
        loop {
            // Check for move down
            let below = (sand_pos.0, sand_pos.1 + 1);
            let diag_left = (sand_pos.0 - 1, sand_pos.1 + 1);
            let diag_right = (sand_pos.0 + 1, sand_pos.1 + 1);

            sand_pos = match (grid.at(&below), grid.at(&diag_left), grid.at(&diag_right)) {
                (None, _, _) => below,
                (Some(_), None, _) => diag_left,
                (Some(_), Some(_), None) => diag_right,
                (Some(_), Some(_), Some(_)) => break,
            };

            if sand_pos.1 > bottom_point {
                break 'outer;
            }
        }
        placed += 1;
        grid.set(sand_pos, true);
    }

    placed
}

fn part2(input: &[Vec<(isize, isize)>]) -> isize {
    let mut grid = SparseGrid::new();
    let mut bottom_point = 0;

    for path in input {
        let mut path_iter = path.iter();
        let mut last = path_iter
            .next()
            .expect("path should be at least 1 long")
            .clone();
        for n in path_iter {
            let dir = ((n.0 - last.0).signum(), (n.1 - last.1).signum());
            grid.set(*n, true);

            while last != *n {
                grid.set(last, true);
                bottom_point = bottom_point.max(last.1);
                last = ((last.0 + dir.0), (last.1 + dir.1));
            }
        }
    }

    for x in -5_000..5_000 {
        grid.set((x, bottom_point + 2), true);
    }

    let sand_drop = (500, 0);
    let mut placed = 0;

    'outer: loop {
        let mut sand_pos = sand_drop.clone();
        loop {
            // Check for move down
            let below = (sand_pos.0, sand_pos.1 + 1);
            let diag_left = (sand_pos.0 - 1, sand_pos.1 + 1);
            let diag_right = (sand_pos.0 + 1, sand_pos.1 + 1);

            sand_pos = match (grid.at(&below), grid.at(&diag_left), grid.at(&diag_right)) {
                (None, _, _) => below,
                (Some(_), None, _) => diag_left,
                (Some(_), Some(_), None) => diag_right,
                (Some(_), Some(_), Some(_)) => break,
            };
        }
        placed += 1;
        if sand_pos == sand_drop {
            break 'outer;
        }
        grid.set(sand_pos, true);
    }

    placed
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_argv_lines().collect::<Result<Vec<String>, _>>()?;
    let input = input
        .iter()
        .map(|s| probe_reports::path(s))
        .collect::<Result<Vec<Vec<(isize, isize)>>, _>>()?;

    println!("{}", part1(&input));
    println!("{}", part2(&input));

    Ok(())
}
