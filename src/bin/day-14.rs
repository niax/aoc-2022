use aoc2022::commons::{
    grid::{Grid, BitGrid},
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

#[derive(Clone)]
struct CaveGrid {
    grid: BitGrid,
    wall_bottom: isize,
}

const OFFSET: isize = 500;

impl CaveGrid {
    pub fn new() -> Self {
        Self {
            grid: BitGrid::new((OFFSET * 4) as usize, OFFSET as usize),
            wall_bottom: isize::MIN,
        }
    }

    pub fn populated(&self, point: &(isize, isize)) -> bool {
        if point.1 >= self.wall_bottom + 2 {
            true
        } else {
            *self.grid.at(&((point.0 + OFFSET) as usize, point.1 as usize)).expect("Point should be in grid")
        }
    }

    pub fn populate(&mut self, point: (isize, isize)) {
        self.grid.set(((point.0 + OFFSET) as usize, point.1 as usize), true);
    }

    pub fn load_path(&mut self, path: &[(isize, isize)]) {
        let mut path_iter = path.iter();
        let mut last = path_iter
            .next()
            .expect("path should be at least 1 long")
            .clone();
        for n in path_iter {
            let dir = ((n.0 - last.0).signum(), (n.1 - last.1).signum());
            self.populate(*n);
            self.wall_bottom = self.wall_bottom.max(n.1);

            while last != *n {
                self.populate(last);
                self.wall_bottom = self.wall_bottom.max(last.1);
                last = ((last.0 + dir.0), (last.1 + dir.1));
            }
        }
    }
}


fn part1(mut input: CaveGrid) -> isize {
    let sand_drop = (500, 0);
    let mut placed = 0;

    'outer: loop {
        let mut sand_pos = sand_drop.clone();
        loop {
            // Check for move down
            let below = (sand_pos.0, sand_pos.1 + 1);
            let diag_left = (sand_pos.0 - 1, sand_pos.1 + 1);
            let diag_right = (sand_pos.0 + 1, sand_pos.1 + 1);

            sand_pos = match (input.populated(&below), input.populated(&diag_left), input.populated(&diag_right)) {
                (false, _, _) => below,
                (true, false, _) => diag_left,
                (true, true, false) => diag_right,
                (true, true, true) => break,
            };

            if sand_pos.1 > input.wall_bottom {
                break 'outer;
            }
        }
        placed += 1;
        input.populate(sand_pos);
    }

    placed
}

fn part2(mut input: CaveGrid) -> isize {
    let sand_drop = (500, 0);
    let mut placed = 0;

    'outer: loop {
        let mut sand_pos = sand_drop.clone();
        loop {
            // Check for move down
            let below = (sand_pos.0, sand_pos.1 + 1);
            let diag_left = (sand_pos.0 - 1, sand_pos.1 + 1);
            let diag_right = (sand_pos.0 + 1, sand_pos.1 + 1);

            sand_pos = match (input.populated(&below), input.populated(&diag_left), input.populated(&diag_right)) {
                (false, _, _) => below,
                (true, false, _) => diag_left,
                (true, true, false) => diag_right,
                (true, true, true) => break,
            };
        }
        placed += 1;
        if sand_pos == sand_drop {
            break 'outer;
        }
        input.populate(sand_pos);
    }

    placed
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_argv_lines().collect::<Result<Vec<String>, _>>()?;
    let mut grid = CaveGrid::new();
    for s in input {
        let path = probe_reports::path(&s)?;
        grid.load_path(&path);
    }

    println!("{}", part1(grid.clone()));
    println!("{}", part2(grid.clone()));

    Ok(())
}
