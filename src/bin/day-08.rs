use aoc2022::commons::grid::{BitGrid, Grid, SingleVecGrid};
use aoc2022::commons::io::load_argv_lines;
use std::error::Error;

fn part1(input: &SingleVecGrid<u8>) -> usize {
    let mut visible = BitGrid::new(input.width(), input.height());

    for x in 0..input.width() {
        // From top in
        let mut max = None;
        for y in 0..input.height() {
            let tree_height = input.at(&(x, y));
            if tree_height > max {
                visible.set((x, y), true);
                max = tree_height;
            }
        }
        // From bottom in
        let mut max = None;
        for y in (0..input.height()).rev() {
            let tree_height = input.at(&(x, y));
            if tree_height > max {
                visible.set((x, y), true);
                max = tree_height;
            }
        }
    }

    for y in 0..input.height() {
        // From left in
        let mut max = None;
        for x in 0..input.width() {
            let tree_height = input.at(&(x, y));
            if tree_height > max {
                visible.set((x, y), true);
                max = tree_height;
            }
        }
        // From right in
        let mut max = None;
        for x in (0..input.width()).rev() {
            let tree_height = input.at(&(x, y));
            if tree_height > max {
                visible.set((x, y), true);
                max = tree_height;
            }
        }
    }

    visible.set_cell_count()
}

fn treehouse_score(grid: &SingleVecGrid<u8>, x: usize, y: usize) -> usize {
    let tree_height = grid.at(&(x, y)).expect("Bad coord");
    let dirs = vec![(0, -1), (-1, 0), (0, 1), (1, 0)];
    dirs.iter()
        .map(|step| {
            let mut score = 0;
            for tree in grid.raycast((x, y), *step).skip(1) {
                score += 1;
                if tree >= tree_height {
                    break;
                }
            }
            score
        })
        .product()
}

fn part2(input: &SingleVecGrid<u8>) -> usize {
    (0..input.height())
        .map(|y| {
            (0..input.width())
                .map(|x| treehouse_score(input, x, y))
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap_or(0)
}

fn parse(input: &[String]) -> Result<SingleVecGrid<u8>, Box<dyn Error>> {
    let mut grid = SingleVecGrid::<u8>::new(input[0].len(), input.len());
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.set((x, y), c.to_digit(10).unwrap() as u8);
        }
    }

    Ok(grid)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_argv_lines::<String>().collect::<Result<Vec<_>, _>>()?;
    let grid = parse(&input)?;

    println!("{}", part1(&grid));
    println!("{}", part2(&grid));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2022::commons::test_helpers::TestCase;

    #[test]
    fn test_solution() {
        let cases = [
            TestCase {
                input_path: "inputs/extra/08.sample",
                part1_expected: 21,
                part2_expected: 8,
            },
            TestCase {
                input_path: "inputs/08",
                part1_expected: 1825,
                part2_expected: 235200,
            },
        ];

        for case in cases {
            let input = case
                .load_file()
                .lines()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            let grid = parse(&input).unwrap();
            assert_eq!(part1(&grid), case.part1_expected);
            assert_eq!(part2(&grid), case.part2_expected);
        }
    }
}
