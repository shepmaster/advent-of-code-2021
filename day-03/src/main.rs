use std::collections::BTreeMap;

const INPUT: &str = include_str!("../input");

fn main() {
    // Not 17640 -- real numbers are > 5 bits
    println!("part1: {}", gamma_epsilon_product(INPUT));
}

fn gamma_epsilon_product(s: &str) -> u64 {
    let mut map = BTreeMap::<_, i32>::new();

    for l in s.lines() {
        let l = l.trim();
        if l.is_empty() {
            continue;
        }
        for (p, c) in l.chars().enumerate() {
            let delta = if c == '1' { 1 } else { -1 };
            *map.entry(p).or_default() += delta;
        }
    }

    let mut gamma = 0u64;
    let mut mask = 0;

    for (_p, c) in map {
        assert_ne!(c, 0, "No majority");

        gamma <<= 1;

        if c > 0 {
            gamma |= 1;
        }

        mask <<= 1;
        mask |= 1;
    }

    let omega = !gamma & mask;
    omega * gamma
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"#;

    #[test]
    fn test_part_1() {
        assert_eq!(198, gamma_epsilon_product(TEST_INPUT));
    }
}
