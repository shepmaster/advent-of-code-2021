#![feature(int_abs_diff)]

use std::collections::BTreeMap;

const INPUT: &str = include_str!("../input");

fn main() {
    println!("part1: {}", minimum_alignment_fuel(INPUT, fuel_cost_linear));
    println!("part2: {}", minimum_alignment_fuel(INPUT, fuel_cost_ramped));
}

fn minimum_alignment_fuel(s: &str, fuel_cost: impl Fn(&Positions, Coord) -> u64) -> u64 {
    minimum_alignment_fuel_inner(s, fuel_cost).unwrap()
}

type Coord = u32;
type Positions = BTreeMap<Coord, u64>;

fn minimum_alignment_fuel_inner(
    s: &str,
    fuel_cost: impl Fn(&Positions, Coord) -> u64,
) -> Result<u64> {
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

fn fuel_cost_linear(positions: &Positions, destination: Coord) -> u64 {
    sum_fuel_computation(positions, |pos| {
        u64::from(Coord::abs_diff(pos, destination))
    })
}

fn fuel_cost_ramped(positions: &Positions, destination: Coord) -> u64 {
    sum_fuel_computation(positions, |pos| {
        let dist = u64::from(Coord::abs_diff(pos, destination));
        inclusive_sum_down_to_zero(dist)
    })
}

fn sum_fuel_computation(positions: &Positions, f: impl Fn(Coord) -> u64) -> u64 {
    positions.iter().map(|(&pos, &count)| f(pos) * count).sum()
}

fn inclusive_sum_down_to_zero(value: u64) -> u64 {
    if value % 2 == 0 {
        (value / 2) * (value + 1)
    } else {
        inclusive_sum_down_to_zero(value - 1) + value
    }
}

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test-input");

    #[test]
    fn test_part1() {
        assert_eq!(37, minimum_alignment_fuel(TEST_INPUT, fuel_cost_linear));
    }

    #[test]
    fn test_part2() {
        assert_eq!(168, minimum_alignment_fuel(TEST_INPUT, fuel_cost_ramped));
    }

    #[test]
    fn test_inclusive_sum_down_to_zero() {
        for v in 0..=100 {
            let oracle: u64 = (0..=v).sum();
            assert_eq!(oracle, inclusive_sum_down_to_zero(v), "For value {v}");
        }
    }
}
