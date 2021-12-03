use itertools::Itertools;

const INPUT: &str = include_str!("../input");

fn main() {
    println!("part 1: {}", number_increases(INPUT));
    println!("part 2: {}", number_window_increases(INPUT));
}

fn number_increases(s: &str) -> usize {
    s.lines()
        .flat_map(|l| l.trim().parse::<u32>())
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

fn number_window_increases(s: &str) -> usize {
    s.lines()
        .flat_map(|l| l.trim().parse::<u32>())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT_1: &str = r#"
199
200
208
210
200
207
240
269
260
263
"#;

    #[test]
    fn test_part1() {
        assert_eq!(7, number_increases(TEST_INPUT_1));
    }

    #[test]
    fn test_part2() {
        assert_eq!(5, number_window_increases(TEST_INPUT_1));
    }
}
