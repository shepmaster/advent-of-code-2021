#![feature(map_first_last)]
#![deny(rust_2018_idioms)]

use std::collections::{BTreeMap, BTreeSet};

const INPUT: &str = include_str!("../input");

fn main() {
    println!("part1: {}", n_paths(INPUT));
    println!("part2: {}", n_paths_one_dupe(INPUT));
}

fn n_paths(s: &str) -> usize {
    n_paths_inner(s).unwrap()
}

fn n_paths_inner(s: &str) -> Result<usize> {
    let graph = parse_graph(s)?;

    Ok(traverse_graph(graph, |candidate, next_node| {
        if next_node == "start" {
            return false;
        }
        if !is_little_cave(next_node) {
            return true;
        }
        !candidate.contains(&next_node)
    })
    .len())
}

fn n_paths_one_dupe(s: &str) -> usize {
    n_paths_one_dupe_inner(s).unwrap()
}

fn n_paths_one_dupe_inner(s: &str) -> Result<usize> {
    let graph = parse_graph(s)?;

    Ok(traverse_graph(graph, |candidate, next_node| {
        if next_node == "start" {
            return false;
        }
        if !is_little_cave(next_node) {
            return true;
        }
        if !has_duplicate_little_cave(candidate) {
            return true;
        }
        !candidate.contains(&next_node)
    })
    .len())
}

type Graph<'a> = BTreeMap<&'a str, BTreeSet<&'a str>>;
type Path<'a> = Vec<&'a str>;

fn parse_graph(s: &str) -> Result<Graph<'_>> {
    let mut graph = BTreeMap::<_, BTreeSet<_>>::new();

    for l in s.trim().lines() {
        let (l, r) = l.split_once("-").ok_or("Malformed line")?;
        graph.entry(l).or_default().insert(r);
        graph.entry(r).or_default().insert(l);
    }

    Ok(graph)
}

fn traverse_graph(
    graph: Graph<'_>,
    allow_next_node: impl Fn(&Path<'_>, &str) -> bool,
) -> BTreeSet<Path<'_>> {
    let mut to_visit = BTreeSet::from_iter([vec!["start"]]);
    let mut paths = BTreeSet::new();

    while let Some(candidate) = to_visit.pop_first() {
        let last = *candidate.last().expect("Path has no components");
        if last == "end" {
            paths.insert(candidate);
        } else {
            for &next_node in &graph[last] {
                if allow_next_node(&candidate, next_node) {
                    let mut next_path = candidate.clone();
                    next_path.push(next_node);
                    to_visit.insert(next_path);
                }
            }
        }
    }

    paths
}

fn has_duplicate_little_cave(candidate: &Path<'_>) -> bool {
    let mut candidate = &candidate[..];

    while let Some((h, next_candidate)) = candidate.split_first() {
        candidate = next_candidate;
        if !is_little_cave(h) {
            continue;
        }
        if candidate.contains(h) {
            return true;
        }
    }

    false
}

fn is_little_cave(node: &str) -> bool {
    node.chars().all(|c| c.is_ascii_lowercase())
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

    #[test]
    fn test_part_2() {
        assert_eq!(36, n_paths_one_dupe(TEST_INPUT_0));
        assert_eq!(103, n_paths_one_dupe(TEST_INPUT_1));
        assert_eq!(3509, n_paths_one_dupe(TEST_INPUT_2));
    }
}
