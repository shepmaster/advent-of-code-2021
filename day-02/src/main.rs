use std::str::FromStr;

const INPUT: &str = include_str!("../input");

fn main() {
    println!("part 1: {}", distance_product(INPUT));
}

fn distance_product(s: &str) -> u64 {
    let mut x = 0;
    let mut y = 0;

    for d in s.lines().flat_map(Direction::from_str) {
        use Direction::*;
        match d {
            Forward(v) => x += v,
            Down(v) => y += v,
            Up(v) => y -= v,
        }
    }

    x * y
}

#[derive(Debug)]
enum Direction {
    Forward(u64),
    Down(u64),
    Up(u64),
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let (k, v) = s.split_once(" ").ok_or(())?;
        let v = v.parse().map_err(drop)?;

        use Direction::*;
        match k {
            "forward" => Ok(Forward(v)),
            "down" => Ok(Down(v)),
            "up" => Ok(Up(v)),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"
forward 5
down 5
forward 8
up 3
down 8
forward 2
"#;

    #[test]
    fn test_part_1() {
        assert_eq!(150, distance_product(TEST_INPUT));
    }
}
