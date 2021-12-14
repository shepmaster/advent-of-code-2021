#![feature(array_windows)]

use std::{collections::BTreeMap, mem};

const INPUT: &str = include_str!("../input");

fn main() {
    println!("part1: {}", difference_of_max_and_min_elements(INPUT, 10));
    println!("part2: {}", difference_of_max_and_min_elements(INPUT, 40));
}

fn difference_of_max_and_min_elements(s: &str, iterations: usize) -> usize {
    let mut lines = s.lines();

    let polymer = lines.next().expect("Missing polymer template");
    let polymer = polymer.trim().as_bytes();
    let mut polymer_pairs = BTreeMap::new();
    for &pair in polymer.array_windows::<2>() {
        *polymer_pairs.entry(pair).or_insert(0usize) += 1;
    }

    let rules: BTreeMap<_, _> = lines
        .skip(1)
        .map(|rule| {
            let (l, r) = rule.trim().split_once("->").expect("Malformed rule");
            let l = l
                .trim()
                .as_bytes()
                .array_windows::<2>()
                .next()
                .expect("Key did not have two elements");
            let r = r
                .trim()
                .as_bytes()
                .get(0)
                .expect("Value did not have one element");
            (*l, *r)
        })
        .collect();

    let mut next_polymer_pairs = BTreeMap::new();
    for _ in 0..iterations {
        for (pair @ &[l, r], &count) in polymer_pairs.iter() {
            let n = rules[pair];
            *next_polymer_pairs.entry([l, n]).or_default() += count;
            *next_polymer_pairs.entry([n, r]).or_default() += count;
        }

        mem::swap(&mut polymer_pairs, &mut next_polymer_pairs);
        next_polymer_pairs.clear();
    }

    difference_of_max_and_min(polymer_pairs, polymer)
}

type Pairs = BTreeMap<[u8; 2], usize>;

fn difference_of_max_and_min(polymer_pairs: Pairs, original_polymer: &[u8]) -> usize {
    let mut frequencies = BTreeMap::new();
    for ([l, r], count) in polymer_pairs {
        *frequencies.entry(l).or_insert(0usize) += count;
        *frequencies.entry(r).or_insert(0usize) += count;
    }

    // We double-count all the elements except the first and the
    // last. Update to double-count all of them.
    let head = *original_polymer.first().expect("original polymer is empty");
    let tail = *original_polymer.last().expect("original polymer is empty");
    *frequencies.entry(head).or_default() += 1;
    *frequencies.entry(tail).or_default() += 1;

    let mut frequencies = Vec::from_iter(frequencies);
    frequencies.sort_unstable_by_key(|&(_, n)| n);

    let &(_min_e, min) = frequencies
        .first()
        .expect("Did not have a minimum element count");
    let &(_max_e, max) = frequencies
        .last()
        .expect("Did not have a maximum element count");

    // Undo the double counting
    (max - min) / 2
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test-input");

    #[test]
    fn test_part_1() {
        assert_eq!(1588, difference_of_max_and_min_elements(TEST_INPUT, 10));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            2188189693529,
            difference_of_max_and_min_elements(TEST_INPUT, 40)
        );
    }
}
