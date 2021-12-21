#![feature(map_first_last)]

use std::{collections::BTreeMap, iter, ops};

const INPUT: &str = include_str!("../input");

fn main() {
    println!("part1: {}", play_game(INPUT));
    println!("part2: {}", play_dirac_game(INPUT));
}

fn play_game(s: &str) -> u32 {
    let players = parse_players(s);
    run_game_loser_score_rolls_product(players)
}

fn play_dirac_game(s: &str) -> u64 {
    let players = parse_players(s);
    run_game_dirac(players)
}

type Players = [(u32, Position); 2];

fn parse_players(s: &str) -> Players {
    let mut positions = s.lines().map(|l| {
        let (_, v) = l.split_once(':').expect("Malformed player line");
        v.trim().parse().expect("Malformed player position")
    });

    let p1 = positions.next().expect("Missing Player 1");
    let p2 = positions.next().expect("Missing Player 2");

    [(0u32, Position::new(p1)), (0u32, Position::new(p2))]
}

fn run_game_loser_score_rolls_product(mut players: Players) -> u32 {
    let die = &mut deterministic_die();
    let mut n_rolls = 0;

    let winning_player = 'game: loop {
        for (player_idx, (score, position)) in players.iter_mut().enumerate() {
            position.extend(die.take(3));
            n_rolls += 3;
            *score += position.value();

            if *score >= 1000 {
                break 'game player_idx;
            }
        }
    };
    let losing_player = 1 - winning_player;

    n_rolls * players[losing_player].0
}

fn run_game_dirac(players: Players) -> u64 {
    let mut universes = BTreeMap::from_iter([((0, players), 1)]);
    let mut wins = [0; 2];

    // (count, final roll)
    let dirac_die = [(1, 3), (3, 4), (6, 5), (7, 6), (6, 7), (3, 8), (1, 9)];

    while let Some(((player_idx, players), count)) = universes.pop_first() {
        for &(die_count, die_value) in &dirac_die {
            let mut players = players.clone();
            let (score, position) = &mut players[player_idx];

            *position += die_value;
            *score += position.value();

            let new_count = count * die_count;

            if *score >= 21 {
                wins[player_idx] += new_count;
            } else {
                let next_player_idx = 1 - player_idx;
                // TODO: probably don't need entry... never repeat this state?
                *universes.entry((next_player_idx, players)).or_default() += new_count;
                // TODO: can we instead cache a mapping of position to end positions and score deltas?
            }
        }
    }

    wins.sort_unstable();
    wins[1]
}

fn deterministic_die() -> impl Iterator<Item = u32> {
    let mut counter = WrappedCounter::<1, 100>::new(1);
    iter::from_fn(move || {
        let v = counter.value();
        counter += 1;
        Some(v)
    })
}

type Position = WrappedCounter<1, 10>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct WrappedCounter<const MIN: u32, const MAX: u32>(u32);

impl<const MIN: u32, const MAX: u32> Default for WrappedCounter<MIN, MAX> {
    fn default() -> Self {
        Self(0)
    }
}

impl<const MIN: u32, const MAX: u32> WrappedCounter<MIN, MAX> {
    const _IS_ORDERED: () = { assert!(MAX > MIN) };

    fn new(v: u32) -> Self {
        assert!((MIN..=MAX).contains(&v));
        Self(v - MIN)
    }

    fn value(&self) -> u32 {
        self.0 + MIN
    }

    fn wrap(&mut self) {
        self.0 %= MAX;
    }
}

impl<const MIN: u32, const MAX: u32> ops::AddAssign<u32> for WrappedCounter<MIN, MAX> {
    fn add_assign(&mut self, rhs: u32) {
        self.0 += rhs;
        self.wrap();
    }
}

impl<const MIN: u32, const MAX: u32> iter::Extend<u32> for WrappedCounter<MIN, MAX> {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = u32>,
    {
        for v in iter {
            *self += v;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test-input");

    #[test]
    fn test_part1() {
        assert_eq!(739785, play_game(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(444356092776315, play_dirac_game(TEST_INPUT));
    }

    #[test]
    fn deterministic_die() {
        let mut die = super::deterministic_die();
        assert!(die.by_ref().take(100).eq(1..=100));
        assert!(die.by_ref().take(100).eq(1..=100));
        assert!(die.by_ref().take(100).eq(1..=100));
    }

    #[test]
    fn wrapped_counter() {
        let mut c = WrappedCounter::<1, 2>::default();
        assert_eq!(1, c.value());

        c += 1;
        assert_eq!(2, c.value());

        c += 1;
        assert_eq!(1, c.value());

        c += 1;
        assert_eq!(2, c.value());

        c += 1;
        assert_eq!(1, c.value());

        c += 2;
        assert_eq!(1, c.value());

        c += 3;
        assert_eq!(2, c.value());
    }
}
