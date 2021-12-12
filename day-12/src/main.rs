#![feature(map_first_last)]

use std::collections::{BTreeMap, BTreeSet};

const INPUT: &str = include_str!("../input");

fn main() {
    println!("part1: {}", n_paths(INPUT));
}

fn n_paths(s: &str) -> usize {
    n_paths_inner(s).unwrap()
}

fn n_paths_inner(s: &str) -> Result<usize> {
    let mut graph = BTreeMap::<_, BTreeSet<_>>::new();

    for l in s.trim().lines() {
        let (l, r) = l.split_once("-").ok_or("Malformed line")?;
        graph.entry(l).or_default().insert(r);
        graph.entry(r).or_default().insert(l);
    }

    let mut to_visit = BTreeSet::from_iter([vec!["start"]]);
    let mut paths = BTreeSet::new();

    while let Some(candidate) = to_visit.pop_first() {
        let last = *candidate.last().expect("Path has no components");
        if last == "end" {
            paths.insert(candidate);
        } else {
            for &next_node in &graph[last] {
                let is_little_cave = next_node.chars().all(|c| c.is_ascii_lowercase());
                if is_little_cave && candidate.contains(&next_node) {
                    continue;
                }

                let mut next_path = candidate.clone();
                next_path.push(next_node);
                to_visit.insert(next_path);
            }
        }
    }

    Ok(paths.len())
}

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT_0: &str = include_str!("../test-input-0");
    const TEST_INPUT_1: &str = include_str!("../test-input-1");
    const TEST_INPUT_2: &str = include_str!("../test-input-2");

    #[test]
    fn test_part_1() {
        assert_eq!(10, n_paths(TEST_INPUT_0));
        assert_eq!(19, n_paths(TEST_INPUT_1));
        assert_eq!(226, n_paths(TEST_INPUT_2));
    }
}
