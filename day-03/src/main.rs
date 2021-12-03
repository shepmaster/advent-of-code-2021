use std::collections::BTreeMap;

const INPUT: &str = include_str!("../input");

fn main() {
    // Not 17640 -- real numbers are > 5 bits
    println!("part1: {}", gamma_epsilon_product(INPUT));
    println!("part2: {}", life_support_rating(INPUT));
}

fn gamma_epsilon_product(s: &str) -> u64 {
    let mut map = BTreeMap::<_, i32>::new();

    for l in clean_lines(s) {
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

fn life_support_rating(s: &str) -> u64 {
    fn delve<'a>(lines: &[&'a str], prefer_one: bool, depth: usize) -> &'a str {
        // Exit if we only have one string
        if let Some((one, rest)) = lines.split_first() {
            if rest.is_empty() {
                return one;
            }
        }

        let (bit_0, bit_1): (Vec<_>, Vec<_>) = lines
            .iter()
            .partition(|l| l.chars().nth(depth) == Some('0'));

        use std::cmp::Ordering::*;
        let selected = match (bit_0.len().cmp(&bit_1.len()), prefer_one) {
            (Less, true) => bit_1,
            (Equal, true) => bit_1,
            (Greater, true) => bit_0,

            (Less, false) => bit_0,
            (Equal, false) => bit_0,
            (Greater, false) => bit_1,
        };

        delve(&selected, prefer_one, depth + 1)
    }

    let lines: Vec<_> = clean_lines(s).collect();
    let oxygen = delve(&lines, true, 0);
    let co2 = delve(&lines, false, 0);

    let oxygen = u64::from_str_radix(oxygen, 2).expect("Not binary");
    let co2 = u64::from_str_radix(co2, 2).expect("Not binary");

    oxygen * co2
}

fn clean_lines(s: &str) -> impl Iterator<Item = &str> {
    s.lines().map(str::trim).filter(|s| !s.is_empty())
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

    #[test]
    fn test_part_2() {
        assert_eq!(230, life_support_rating(TEST_INPUT));
    }
}
