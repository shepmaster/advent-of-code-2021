#![feature(map_first_last)]

use itertools::Itertools;
use std::{
    cmp::max,
    collections::{BTreeMap, BTreeSet},
};

const INPUT: &str = include_str!("../input");

fn main() {
    // WRONG: 600 (too low) -- was using score of exit, not entry
    println!("part1: {}", path_risk(INPUT));
    // WRONG: 2944 (too high) -- was only allowing data to flow right/down, not form loops
    println!("part2: {}", path_risk_scaled(INPUT));
}

fn path_risk(s: &str) -> Risk {
    path_risk_inner(s).unwrap()
}

fn path_risk_inner(s: &str) -> Result<Risk> {
    let (grid, x_max, y_max) = parse_grid(s)?;
    let end = (x_max, y_max);
    Ok(cost(&grid, end, x_max, y_max))
}

fn path_risk_scaled(s: &str) -> Risk {
    path_risk_scaled_inner(s).unwrap()
}

fn path_risk_scaled_inner(s: &str) -> Result<Risk> {
    let (grid, x_max, y_max) = parse_grid(s)?;
    let (grid, x_max, y_max) = scale_grid(grid, x_max, y_max);
    let end = (x_max, y_max);

    Ok(cost(&grid, end, x_max, y_max))
}

type Coord = (usize, usize);
type Risk = u32;
type Grid = BTreeMap<Coord, Risk>;

fn parse_grid(s: &str) -> Result<(Grid, usize, usize)> {
    let mut x_max = 0;
    let mut y_max = 0;
    let mut grid = Grid::new();

    for (y, line) in s.lines().enumerate() {
        y_max = max(y, y_max);
        for (x, c) in line.trim().chars().enumerate() {
            x_max = max(x, x_max);
            let v = c.to_digit(10).ok_or("Invalid risk value")?;
            grid.insert((x, y), v);
        }
    }

    Ok((grid, x_max, y_max))
}

fn cost(grid: &Grid, coord: Coord, x_max: usize, y_max: usize) -> Risk {
    let mut costs = Grid::new();
    let mut to_visit = BTreeSet::from_iter([(0, 0)]);

    while let Some(coord) = to_visit.pop_first() {
        let risk = grid[&coord];

        let current_min_cost = neighbors(coord, x_max, y_max)
            .flat_map(|neighbor_coord| costs.get(&neighbor_coord))
            .map(|&cost| cost + risk)
            .min();

        match current_min_cost {
            Some(current_min_cost) => {
                if costs.get(&coord).map_or(true, |&c| current_min_cost < c) {
                    costs.insert(coord, current_min_cost);
                    to_visit.extend(neighbors(coord, x_max, y_max));
                }
            }
            None => {
                costs.insert(coord, 0);
                to_visit.extend(neighbors(coord, x_max, y_max));
            }
        }
    }

    costs[&coord]
}

fn neighbors((x, y): Coord, x_max: usize, y_max: usize) -> impl Iterator<Item = Coord> {
    let left = x.checked_sub(1).map(|x| (x, y));
    let right = x.checked_add(1).map(|x| (x, y));
    let up = y.checked_sub(1).map(|y| (x, y));
    let down = y.checked_add(1).map(|y| (x, y));

    [left, up, right, down]
        .into_iter()
        .flatten()
        .filter(move |&(x, y)| x <= x_max && y <= y_max)
}

const SCALE_FACTOR: usize = 5;

fn scale_grid(mut grid: Grid, x_max: usize, y_max: usize) -> (Grid, usize, usize) {
    let mut x_width = x_max + 1;
    let mut y_width = y_max + 1;

    let mut copy = |from, to| {
        let v = grid[&from];
        let v2 = v + 1;
        let v2 = if v2 > 9 { 1 } else { v2 };

        grid.insert(to, v2);
    };

    for (prev_factor, next_factor) in (0..SCALE_FACTOR).tuple_windows() {
        let prev_offset = x_width * prev_factor;
        let next_offset = x_width * next_factor;

        for (x, y) in (0..x_width).cartesian_product(0..y_width) {
            let from = (x + prev_offset, y);
            let to = (x + next_offset, y);
            copy(from, to);
        }
    }
    x_width *= SCALE_FACTOR;

    for (prev_factor, next_factor) in (0..SCALE_FACTOR).tuple_windows() {
        let prev_offset = y_width * prev_factor;
        let next_offset = y_width * next_factor;

        for (x, y) in (0..x_width).cartesian_product(0..y_width) {
            let from = (x, y + prev_offset);
            let to = (x, y + next_offset);
            copy(from, to);
        }
    }
    y_width *= SCALE_FACTOR;

    (grid, x_width - 1, y_width - 1)
}

#[allow(dead_code)]
fn print_grid(grid: &Grid) {
    let x_max = *grid.keys().map(|(x, _)| x).max().unwrap();
    let y_max = *grid.keys().map(|(_, y)| y).max().unwrap();

    for y in 0..=y_max {
        for x in 0..=x_max {
            print!("{:04} ", grid[&(x, y)]);
        }
        println!()
    }
}

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test-input");
    const TEST_INPUT_SCALED: &str = include_str!("../test-input-scaled");

    #[test]
    fn test_part_1() {
        assert_eq!(40, path_risk(TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(315, path_risk_scaled(TEST_INPUT));
    }

    #[test]
    fn test_scaling() {
        let (grid, x_max, y_max) = parse_grid(TEST_INPUT).unwrap();
        let (grid, x_max, y_max) = scale_grid(grid, x_max, y_max);

        let (scaled_grid, scaled_x_max, scaled_y_max) = parse_grid(TEST_INPUT_SCALED).unwrap();

        assert_eq!(scaled_x_max, x_max);
        assert_eq!(scaled_y_max, y_max);
        assert_eq!(scaled_grid.len(), grid.len());
        assert_eq!(scaled_grid, grid);
    }
}
