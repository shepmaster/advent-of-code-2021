use itertools::Itertools;
use std::{collections::BTreeMap, iter};

const INPUT: &str = include_str!("../input");

fn main() {
    println!("part1: {}", number_overlapping_points(INPUT));
}

type Coord = (u64, u64);
type Grid = BTreeMap<Coord, usize>;

fn number_overlapping_points(s: &str) -> usize {
    let lines = s.lines().flat_map(|l| {
        let (l, r) = l.split_once("->")?;

        let (x1, y1) = l.trim().split_once(",")?;
        let (x2, y2) = r.trim().split_once(",")?;

        let x1 = x1.parse().ok()?;
        let y1 = y1.parse().ok()?;
        let x2 = x2.parse().ok()?;
        let y2 = y2.parse().ok()?;

        Some(((x1, y1), (x2, y2)))
    });

    let mut grid = Grid::default();
    for ((x1, y1), (x2, y2)) in lines {
        let mut vertical;
        let mut horizontal;
        let mut diagonal;

        let coords: &mut dyn Iterator<Item = Coord> = if x1 == x2 {
            let xs = iter::repeat(x1);
            let ys = increasing_range_inclusive(y1, y2);
            vertical = xs.zip(ys);

            &mut vertical
        } else if y1 == y2 {
            let xs = increasing_range_inclusive(x1, x2);
            let ys = iter::repeat(y1);
            horizontal = xs.zip(ys);

            &mut horizontal
        } else {
            // not yet undefined
            diagonal = iter::empty();

            &mut diagonal
        };

        for coord in coords {
            *grid.entry(coord).or_default() += 1;
        }
    }

    // print_grid(&grid);
    grid.values().filter(|&&c| c >= 2).count()
}

fn increasing_range_inclusive<T>(y1: T, y2: T) -> std::ops::RangeInclusive<T>
where
    T: PartialOrd,
{
    if y1 < y2 {
        y1..=y2
    } else {
        y2..=y1
    }
}

#[allow(unused)]
fn print_grid(grid: &Grid) {
    let (x_min, x_max) = grid.keys().map(|(x, _)| *x).minmax().into_option().unwrap();
    let (y_min, y_max) = grid.keys().map(|(_, y)| *y).minmax().into_option().unwrap();

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            match grid.get(&(x, y)) {
                Some(v) => print!("{:02} ", v),
                None => print!(".. "),
            }
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test-input");

    #[test]
    fn test_part_1() {
        assert_eq!(5, number_overlapping_points(TEST_INPUT));
    }
}
