use std::{cmp::max, collections::BTreeMap};

const INPUT: &str = include_str!("../input");

fn main() {
    println!("part1: {}", risk_level_sum(INPUT));
}

fn risk_level_sum(s: &str) -> u32 {
    risk_level_sum_inner(s).unwrap()
}

fn risk_level_sum_inner(s: &str) -> Result<u32> {
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

    let minimums = itertools::iproduct!(0..=max_x, 0..=max_y).filter_map(|(x, y)| {
        let v = board[&(x, y)];

        let l = x.checked_sub(1).and_then(|x| board.get(&(x, y))).copied();
        let r = x.checked_add(1).and_then(|x| board.get(&(x, y))).copied();
        let u = y.checked_sub(1).and_then(|y| board.get(&(x, y))).copied();
        let d = y.checked_add(1).and_then(|y| board.get(&(x, y))).copied();

        let is_minimum = [l, r, u, d].into_iter().all(|t| t.map_or(true, |t| t > v));

        is_minimum.then(|| v)
    });

    Ok(minimums.map(|v| v + 1).sum())
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
}
