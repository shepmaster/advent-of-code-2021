#![feature(array_windows)]

use itertools::Itertools;
use std::{collections::BTreeMap, mem};

const INPUT: &str = include_str!("../input");

fn main() {
    println!("part1: {}", difference_of_max_and_min_elements(INPUT, 10));
}

fn difference_of_max_and_min_elements(s: &str, iterations: usize) -> usize {
    let mut lines = s.lines();

    let polymer = lines.next().expect("Missing polymer template");
    let mut polymer = polymer.trim().to_string().into_bytes();

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

    let mut next_polymer = Vec::new();
    for _ in 0..iterations {
        let expanded = polymer
            .iter()
            .copied()
            .interleave(polymer.array_windows().map(|k| rules[k]));
        next_polymer.extend(expanded);
        mem::swap(&mut polymer, &mut next_polymer);
        next_polymer.clear();
    }

    let mut frequencies = BTreeMap::<_, usize>::new();
    for element in polymer {
        *frequencies.entry(element).or_default() += 1;
    }

    let mut frequencies = Vec::from_iter(frequencies);
    frequencies.sort_unstable_by_key(|&(_, n)| n);

    let &(_min_e, min) = frequencies
        .first()
        .expect("Did not have a minimum element count");
    let &(_max_e, max) = frequencies
        .last()
        .expect("Did not have a maximum element count");

    max - min
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test-input");

    #[test]
    fn test_part_1() {
        assert_eq!(1588, difference_of_max_and_min_elements(TEST_INPUT, 10));
    }
}
