use itertools::Itertools;

const INPUT: &str = include_str!("../input");

fn main() {
    println!("{}", number_increases(INPUT));
}

fn number_increases(s: &str) -> usize {
    s.lines()
        .flat_map(|l| l.trim().parse::<u32>())
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
    fn test1() {
        assert_eq!(7, number_increases(TEST_INPUT_1));
    }
}
