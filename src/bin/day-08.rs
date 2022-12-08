use aoc2022::commons::io::load_argv_lines;
use std::error::Error;
use aoc2022::commons::grid::{Grid, SingleVecGrid, BitGrid};

fn is_visible(grid: &SingleVecGrid<u8>, x: usize, y: usize) -> bool {
    let tree_height = grid.at(&(x,y)).expect("Bad coord");
    let west = (0..x).map(|x| grid.at(&(x, y))).max().flatten();
    let east = (x+1..grid.width()).map(|x| grid.at(&(x, y))).max().flatten();
    let north = (0..y).map(|y| grid.at(&(x, y))).max().flatten();
    let south = (y+1..grid.height()).map(|y| grid.at(&(x, y))).max().flatten();


    [west, east, north, south].iter().any(|dir| {
        match dir {
            None => true,
            Some(height) => tree_height > height,
        }
    })
}

fn part1(input: &SingleVecGrid<u8>) -> usize {
    let mut visible = BitGrid::new(input.width(), input.height());

    for x in 0..input.width() {
        for y in 0..input.width() {
            visible.set((x, y), is_visible(input, x, y));
        }
    }

    visible.set_cell_count()
}

fn treehouse_score(grid: &SingleVecGrid<u8>, x: usize, y: usize) -> usize {
    let tree_height = grid.at(&(x,y)).expect("Bad coord");

    let mut north = 0;
    for y in (0..y).rev() {
        let tree = grid.at(&(x, y));
        if let Some(height) = tree {
            north += 1;
            if height >= tree_height {
                break;
            }
        }
    }

    let mut west = 0;
    for x in (0..x).rev() {
        let tree = grid.at(&(x, y));
        if let Some(height) = tree {
            west += 1;
            if height >= tree_height {
                break;
            }
        }
    }

    let mut east = 0;
    for x in x+1..grid.width() {
        let tree = grid.at(&(x, y));
        if let Some(height) = tree {
            east += 1;
            if height >= tree_height {
                break;
            }
        }
    }

    let mut south = 0;
    for y in y+1..grid.height() {
        let tree = grid.at(&(x, y));
        if let Some(height) = tree {
            south += 1;
            if height >= tree_height {
                break;
            }
        }
    }
    west * east * north * south
}

fn part2(input: &SingleVecGrid<u8>) -> usize {
    (0..input.height()).map(|y| {
        (0..input.width()).map(|x| {
            treehouse_score(input, x, y)
        }).max().unwrap_or(0)
    }).max().unwrap_or(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_argv_lines::<String>().collect::<Result<Vec<_>, _>>()?;

    let mut grid = SingleVecGrid::<u8>::new(input[0].len(), input.len());
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.set((x, y), c.to_string().parse()?);
        }
    }

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
                part1_expected: 0,
                part2_expected: 0,
            },
            TestCase {
                input_path: "inputs/08",
                part1_expected: 0,
                part2_expected: 0,
            },
        ];

        for case in cases {
            let input = case
                .load_file()
                .lines()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>();
            assert_eq!(part1(&input), case.part1_expected);
            assert_eq!(part2(&input), case.part2_expected);
        }
    }
}
