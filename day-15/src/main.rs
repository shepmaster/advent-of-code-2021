use std::{cmp::max, collections::BTreeMap};

const INPUT: &str = include_str!("../input");

fn main() {
    // WRONG: 600 (too low) -- was using score of exit, not entry
    println!("part1: {}", path_risk(INPUT));
}

fn path_risk(s: &str) -> Risk {
    path_risk_inner(s).unwrap()
}

fn path_risk_inner(s: &str) -> Result<Risk> {
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

    let end = (x_max, y_max);
    Ok(cost(&grid, end))
}

type Coord = (usize, usize);
type Risk = u32;
type Grid = BTreeMap<Coord, Risk>;

fn cost(grid: &Grid, coord: Coord) -> Risk {
    fn cost_inner(memo: &mut Grid, grid: &Grid, coord: Coord) -> Risk {
        if let Some(&tot) = memo.get(&coord) {
            return tot;
        }

        let risk = grid[&coord];

        let min_total_cost = neighbors_back(coord)
            .map(|neighbor_coord| cost_inner(memo, grid, neighbor_coord) + risk)
            .min()
            .unwrap_or_default();

        memo.insert(coord, min_total_cost);
        min_total_cost
    }

    cost_inner(&mut Grid::new(), grid, coord)
}

fn neighbors_back((x, y): Coord) -> impl Iterator<Item = Coord> {
    let left = x.checked_sub(1).map(|x| (x, y));
    let up = y.checked_sub(1).map(|y| (x, y));
    [left, up].into_iter().flatten()
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

    #[test]
    fn test_part_1() {
        assert_eq!(40, path_risk(TEST_INPUT));
    }
}
