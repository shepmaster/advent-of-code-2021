#![feature(map_first_last)]

use std::collections::{BTreeMap, BTreeSet};

const INPUT: &str = include_str!("../input");

fn main() {
    println!("part1: {}", n_flashes(INPUT, 1000));
    println!("part2: {}", first_simultaneous_flash(INPUT));
}

fn n_flashes(s: &str, iterations: usize) -> usize {
    n_flashes_inner(s, iterations).unwrap()
}

fn n_flashes_inner(s: &str, iterations: usize) -> Result<usize> {
    let mut board = parse_board(s)?;
    Ok((0..iterations).map(|_| step(&mut board).len()).sum())
}

fn first_simultaneous_flash(s: &str) -> usize {
    first_simultaneous_flash_inner(s).unwrap()
}

fn first_simultaneous_flash_inner(s: &str) -> Result<usize> {
    let mut board = parse_board(s)?;
    (1..)
        .find(|_| step(&mut board).len() == board.len())
        .ok_or_else(|| "Never found a simultaneous flash".into())
}

type Coord = (usize, usize);
type Board = BTreeMap<Coord, u32>;

fn parse_board(s: &str) -> Result<Board> {
    s.trim()
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.trim().chars().enumerate().map(move |(x, c)| {
                let c = c.to_digit(10)?;
                Some(((x, y), c))
            })
        })
        .collect::<Option<_>>()
        .ok_or_else(|| "Invalid digit".into())
}

fn step(board: &mut Board) -> BTreeSet<Coord> {
    let mut intermediate_flashed = BTreeSet::new();
    let mut flashed = BTreeSet::new();

    for (&c, v) in &mut *board {
        *v += 1;
        if *v > 9 {
            intermediate_flashed.insert(c);
        }
    }

    while let Some(c) = intermediate_flashed.pop_first() {
        flashed.insert(c);

        for n in neighbors(c) {
            if let Some(v) = board.get_mut(&n) {
                *v += 1;
                if *v > 9 && !flashed.contains(&n) {
                    intermediate_flashed.insert(n);
                }
            }
        }
    }

    for c in &flashed {
        if let Some(v) = board.get_mut(c) {
            *v = 0;
        }
    }

    flashed
}

fn neighbors((x, y): Coord) -> impl Iterator<Item = Coord> {
    let x0 = x.checked_sub(1);
    let x1 = Some(x);
    let x2 = x.checked_add(1);
    let y0 = y.checked_sub(1);
    let y1 = Some(y);
    let y2 = y.checked_add(1);

    itertools::chain!(
        x0.and_then(|x| Some((x, y0?))),
        x0.and_then(|x| Some((x, y1?))),
        x0.and_then(|x| Some((x, y2?))),
        x1.and_then(|x| Some((x, y0?))),
        x1.and_then(|x| Some((x, y2?))),
        x2.and_then(|x| Some((x, y0?))),
        x2.and_then(|x| Some((x, y1?))),
        x2.and_then(|x| Some((x, y2?))),
    )
}

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test-input");

    #[test]
    fn test_part_1() {
        assert_eq!(1656, n_flashes(TEST_INPUT, 100));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(195, first_simultaneous_flash(TEST_INPUT));
    }
}
