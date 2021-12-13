use std::{cmp::max, collections::BTreeSet};

const INPUT: &str = include_str!("../input");

fn main() {
    println!("part1: {}", dots_visible_after_folds(INPUT, 1));

    // WRONG: BLKJRBAE; BLKJRBAC
    // Wasn't including max_x, so right-side was cut off
    println!("part2:");
    dots_picture(INPUT);
}

fn dots_visible_after_folds(s: &str, limit: usize) -> usize {
    let mut lines = s.lines();

    let mut grid = parse_grid(&mut lines);
    fold_paper(&mut grid, lines.take(limit));

    grid.len()
}

fn dots_picture(s: &str) {
    let mut lines = s.lines();

    let mut grid = parse_grid(&mut lines);
    fold_paper(&mut grid, lines);

    print_grid(&grid);
}

type Grid = BTreeSet<(i32, i32)>;

fn parse_grid<'a>(lines: impl Iterator<Item = &'a str>) -> Grid {
    lines
        .map(|l| l.trim())
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (x, y) = l.split_once(",").expect("malformed coordinate");
            let x = x.parse().expect("malformed x");
            let y = y.parse().expect("malformed y");
            (x, y)
        })
        .collect()
}

fn fold_paper<'a>(grid: &mut Grid, lines: impl Iterator<Item = &'a str>) {
    let mut to_move = Vec::with_capacity(grid.len());

    for fold in lines {
        let fold = fold.trim_start_matches("fold along ");

        let (dimension, value) = fold.trim().split_once("=").expect("malformed split");
        let value = value.parse().expect("malformed split value");

        to_move.clear();
        match dimension.trim() {
            "x" => {
                to_move.extend(grid.iter().filter(|&(x, _)| x > &value));
                for moved in &to_move {
                    grid.remove(moved);
                }
                for (mut x, y) in to_move.drain(..) {
                    x = 2 * value - x;
                    grid.insert((x, y));
                }
            }
            "y" => {
                to_move.extend(grid.iter().filter(|&(_, y)| y > &value));
                for moved in &to_move {
                    grid.remove(moved);
                }
                for (x, mut y) in to_move.drain(..) {
                    y = 2 * value - y;
                    grid.insert((x, y));
                }
            }
            other => panic!("Unknown dimension {}", other),
        }
    }
}

fn print_grid(grid: &Grid) {
    let mut max_x = 0;
    let mut max_y = 0;

    for &(x, y) in grid {
        max_x = max(x, max_x);
        max_y = max(y, max_y);
    }

    for y in 0..=max_y {
        for x in 0..=max_x {
            let c = if grid.contains(&(x, y)) { '#' } else { ' ' };
            print!("{c}");
        }
        println!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test-input");

    #[test]
    fn test_part_1() {
        assert_eq!(17, dots_visible_after_folds(TEST_INPUT, 1));
    }
}
