#![feature(int_abs_diff)]

use std::collections::BTreeMap;

const INPUT: &str = include_str!("../input");

fn main() {
    println!("part1: {}", minimum_alignment_fuel(INPUT));
}

fn minimum_alignment_fuel(s: &str) -> u64 {
    minimum_alignment_fuel_inner(s).unwrap()
}

type Coord = u32;
type Positions = BTreeMap<Coord, u64>;

fn minimum_alignment_fuel_inner(s: &str) -> Result<u64> {
    let mut positions = Positions::new();
    for pos in s.split(',').map(|p| p.trim().parse()) {
        let pos = pos?;
        *positions.entry(pos).or_default() += 1;
    }

    let &min = positions
        .keys()
        .next()
        .ok_or("Need at least one position")?;
    let &max = positions
        .keys()
        .next_back()
        .ok_or("Need at least one position")?;

    (min..=max)
        .map(|destination| fuel_cost(&positions, destination))
        .min()
        .ok_or("Need at least one position")
        .map_err(Into::into)
}

fn fuel_cost(positions: &Positions, destination: Coord) -> u64 {
    positions
        .iter()
        .map(|(pos, count)| u64::from(Coord::abs_diff(*pos, destination)) * count)
        .sum()
}

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test-input");

    #[test]
    fn test_part1() {
        assert_eq!(37, minimum_alignment_fuel(TEST_INPUT));
    }
}
