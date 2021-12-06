const INPUT: &str = include_str!("../input");

fn main() {
    // Wrong: 385449 (too low) -- last number wasn't being parsed
    println!("part1: {}", simulate_lanternfish(INPUT, 80));
    println!("part2: {}", simulate_lanternfish(INPUT, 256));
}

fn simulate_lanternfish(s: &str, n_days: usize) -> usize {
    let mut days = [0; 9];

    for timer in s.split(',').flat_map(|d| d.trim().parse::<usize>()) {
        days[timer] += 1;
    }

    for _ in 0..n_days {
        let num_zero = days[0];
        days.rotate_left(1);
        days[6] += num_zero; // The fish continues
        days[8] = num_zero; // And makes a new fish
    }

    days.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test-input");

    #[test]
    fn test_part_1_18_days() {
        assert_eq!(26, simulate_lanternfish(TEST_INPUT, 18));
    }

    #[test]
    fn test_part_1_80_days() {
        assert_eq!(5934, simulate_lanternfish(TEST_INPUT, 80));
    }

    #[test]
    fn test_part_2_256_days() {
        assert_eq!(26984457539, simulate_lanternfish(TEST_INPUT, 256));
    }
}
