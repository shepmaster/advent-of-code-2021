#![feature(map_first_last)]

use std::{
    cmp::max,
    collections::{BTreeMap, BTreeSet},
};

const INPUT: &str = include_str!("../input");

fn main() {
    println!("part1: {}", risk_level_sum(INPUT));
    println!("part1: {}", top_three_basin_size_products(INPUT));
}

fn risk_level_sum(s: &str) -> Height {
    risk_level_sum_inner(s).unwrap()
}

fn risk_level_sum_inner(s: &str) -> Result<Height> {
    let (board, max_x, max_y) = parse_board(s)?;

    Ok(minimums(&board, max_x, max_y).map(|(_, v)| v + 1).sum())
}

fn top_three_basin_size_products(s: &str) -> usize {
    top_three_basin_size_products_inner(s).unwrap()
}

fn top_three_basin_size_products_inner(s: &str) -> Result<usize> {
    let (board, max_x, max_y) = parse_board(s)?;
    let minimums = minimums(&board, max_x, max_y).map(|(c, _)| c);

    let mut sizes: Vec<_> = minimums.map(|c| basin_size(&board, c)).collect();
    sizes.sort_unstable();
    Ok(sizes.iter().rev().take(3).product())
}

type Coord = (usize, usize);
type Height = u32;
type Board = BTreeMap<Coord, Height>;

const MAX_HEIGHT: Height = 9;

fn parse_board(s: &str) -> Result<(Board, usize, usize)> {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut board = BTreeMap::new();

    for (y, l) in s.trim().lines().enumerate() {
        for (x, c) in l.trim().chars().enumerate() {
            let c = c.to_digit(10).ok_or("Invalid digit")?;
            board.insert((x, y), c);
            max_x = max(max_x, x);
        }
        max_y = max(max_y, y);
    }

    Ok((board, max_x, max_y))
}

fn minimums(
    board: &Board,
    max_x: usize,
    max_y: usize,
) -> impl Iterator<Item = (Coord, Height)> + '_ {
    itertools::iproduct!(0..=max_x, 0..=max_y).filter_map(move |c| {
        let v = board[&c];
        let is_minimum = neighbors(board, c).all(|(_, t)| t > v);
        is_minimum.then(|| (c, v))
    })
}

fn neighbors(board: &Board, (x, y): Coord) -> impl Iterator<Item = (Coord, Height)> {
    let join_x = |y| board.get(&(x, y)).map(|&v| ((x, y), v));
    let join_y = |x| board.get(&(x, y)).map(|&v| ((x, y), v));

    let u = y.checked_sub(1).and_then(join_x);
    let d = y.checked_add(1).and_then(join_x);
    let l = x.checked_sub(1).and_then(join_y);
    let r = x.checked_add(1).and_then(join_y);

    itertools::chain!(u, d, l, r)
}

fn basin_size(board: &Board, start: Coord) -> usize {
    let mut to_visit = BTreeSet::from_iter([start]);
    let mut visited = BTreeSet::new();

    while let Some(c) = to_visit.pop_first() {
        visited.insert(c);

        for (nc, nv) in neighbors(board, c) {
            if nv != MAX_HEIGHT && !visited.contains(&nc) {
                to_visit.insert(nc);
            }
        }
    }

    visited.len()
}

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test-input");

    #[test]
    fn test_part_1() {
        assert_eq!(15, risk_level_sum(TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(1134, top_three_basin_size_products(TEST_INPUT));
    }
}
