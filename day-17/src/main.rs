use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../input");

fn main() {
    // WRONG: 1830 (too low) --
    // 7503
    println!("part1: {}", maximum_height(INPUT));
}

fn maximum_height(s: &str) -> i32 {
    let target = parse_target(s);
    let paths = valid_paths(target);
    paths.into_iter().flatten().map(|(_, y)| y).max().unwrap()
}

type Coord = (i32, i32);
type Path = Vec<Coord>;
type Target = (RangeInclusive<i32>, RangeInclusive<i32>);

fn parse_target(s: &str) -> Target {
    let s = s.trim().trim_start_matches("target area: ");
    let (x, y) = s.split_once(',').expect("invalid target specification");
    (parse_range(x), parse_range(y))
}

// TODO: inclusive or not?
fn parse_range(x: &str) -> RangeInclusive<i32> {
    let (_, x) = x.split_once('=').expect("invalid x specification");
    let (x0, x1) = x.split_once("..").expect("invalid x range");
    let x0 = x0.parse().expect("invalid x0 value");
    let x1 = x1.parse().expect("invalid x1 value");
    x0..=x1
}

fn valid_paths(target: Target) -> Vec<Path> {
    let mut paths = vec![];
    for x in 0..=*target.0.end() {
        for y in 0.. {
            let velocity = (x, y);

            // Y velocity when we return to y == 0 will cause us to skip over the target area
            if -(velocity.1 + 1) < *target.1.start() {
                break;
            }

            paths.extend(launch(velocity, target.clone()));
        }
    }
    paths
}

fn launch(mut velocity: (i32, i32), target: Target) -> Option<Path> {
    let mut position = (0, 0);
    let mut steps = Vec::new();

    loop {
        steps.push(position);

        let inside_x = target.0.contains(&position.0);
        let inside_y = target.1.contains(&position.1);

        if inside_x && inside_y {
            return Some(steps);
        }

        // Didn't make it to target
        if velocity.0 == 0 && position.0 < *target.0.start() {
            return None;
        }
        // Went past target
        if velocity.0 == 0 && position.0 > *target.0.end() {
            return None;
        }
        // Underneath target
        if velocity.1 < 0 && position.1 < *target.1.start() {
            return None;
        }

        position.0 += velocity.0;
        position.1 += velocity.1;

        // Drag
        velocity.0 -= velocity.0.signum();
        // Gravity
        velocity.1 -= 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test-input");

    #[test]
    fn test_part1() {
        assert_eq!(45, maximum_height(TEST_INPUT));
    }
}
