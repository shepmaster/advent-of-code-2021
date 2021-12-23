use std::{
    cmp::{max, min},
    collections::BTreeSet,
    fmt, mem,
    ops::Range,
};

const INPUT: &str = include_str!("../input");

fn main() {
    println!("part1: {}", n_cubes_on_restricted(INPUT));
}

fn n_cubes_on_restricted(s: &str) -> usize {
    let mut forest = Forest::default();
    for mut area in parse_areas(s) {
        area.clamp_to(-50..51);
        forest.insert(area);
    }
    forest.cubes_on()
}

type Coord = (i32, i32, i32);
type Dimension = Range<i32>;

#[derive(Debug, Clone)]
struct Area {
    mode: bool,
    space: Space,
}

fn parse_areas(s: &str) -> impl Iterator<Item = Area> + '_ {
    s.lines().map(|l| {
        let (mode, l) = l.trim().split_once(' ').expect("Could not find mode");
        let mode = mode == "on";

        let mut coords = l.trim().split(',');

        let mut one_range = || {
            let x = coords.next().expect("missing coord");
            let (_, x) = x.split_once('=').expect("malformed coord");
            let (l, r) = x.split_once("..").expect("malformed coord");
            let l = l.parse::<i32>().expect("invalid coord");
            let r = r.parse::<i32>().expect("invalid coord");
            l..(r + 1) // Adjusting upward for inclusive range
        };

        let x = one_range();
        let y = one_range();
        let z = one_range();
        let space = Space { x, y, z };

        Area { mode, space }
    })
}

impl Area {
    fn clamp_to(&mut self, arg: Range<i32>) {
        self.space.clamp_to(arg);
    }
}

#[derive(Debug, Default)]
struct Forest(Vec<Space>);

impl Forest {
    fn insert(&mut self, a: Area) {
        let spaces = mem::take(&mut self.0);

        self.0 = spaces
            .into_iter()
            .flat_map(|s| s.subtract(&a.space))
            .collect();

        if a.mode {
            self.0.push(a.space)
        }
    }

    fn cubes_on(&self) -> usize {
        self.0.iter().map(Space::volume).sum()
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Space {
    x: Dimension,
    y: Dimension,
    z: Dimension,
}

impl fmt::Debug for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { x, y, z } = self;
        write!(f, "Space({x:?}, {y:?}, {z:?})")
    }
}

impl PartialOrd for Space {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Space {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_cmp_key().cmp(&other.to_cmp_key())
    }
}

impl Space {
    fn new(x: Dimension, y: Dimension, z: Dimension) -> Self {
        Self { x, y, z }
    }

    fn to_cmp_key(&self) -> [i32; 6] {
        let Self { x, y, z } = self;
        [x.start, x.end, y.start, y.end, z.start, z.end]
    }

    fn subtract(&self, other: &Self) -> Vec<Self> {
        let mut result = BTreeSet::from_iter([self.clone()]);

        for corner in other.corners() {
            result = result
                .into_iter()
                .flat_map(|s| s.split_at(corner))
                .filter(|s| !s.is_empty())
                .filter(|s| !s.completely_contains(other))
                .collect();
        }

        result.into_iter().collect()
    }

    fn split_at(&self, coord: Coord) -> [Self; 8] {
        let Self { x, y, z } = self;
        let xu = x.end;
        let xd = x.start;
        let yu = y.end;
        let yd = y.start;
        let zu = z.end;
        let zd = z.start;

        let (x, y, z) = coord;
        let x0 = min(xu, x);
        let x1 = max(xd, x);
        let y0 = min(yu, y);
        let y1 = max(yd, y);
        let z0 = min(zu, z);
        let z1 = max(zd, z);

        [
            Self::new(xd..x0, yd..y0, zd..z0),
            Self::new(xd..x0, yd..y0, z1..zu),
            Self::new(xd..x0, y1..yu, zd..z0),
            Self::new(xd..x0, y1..yu, z1..zu),
            Self::new(x1..xu, yd..y0, zd..z0),
            Self::new(x1..xu, yd..y0, z1..zu),
            Self::new(x1..xu, y1..yu, zd..z0),
            Self::new(x1..xu, y1..yu, z1..zu),
        ]
    }

    fn completely_contains(&self, other: &Self) -> bool {
        let Self { x, y, z } = self;

        let x = x.start >= other.x.start && x.end <= other.x.end;
        let y = y.start >= other.y.start && y.end <= other.y.end;
        let z = z.start >= other.z.start && z.end <= other.z.end;

        x && y && z
    }

    fn is_empty(&self) -> bool {
        self.volume() == 0
    }

    fn volume(&self) -> usize {
        let Self { x, y, z } = self;
        x.len() * y.len() * z.len()
    }

    fn corners(&self) -> impl Iterator<Item = Coord> {
        let Self { x, y, z } = self;
        let xu = x.end;
        let xd = x.start;
        let yu = y.end;
        let yd = y.start;
        let zu = z.end;
        let zd = z.start;

        [
            (xu, yu, zu),
            (xu, yu, zd),
            (xu, yd, zu),
            (xu, yd, zd),
            (xd, yu, zu),
            (xd, yu, zd),
            (xd, yd, zu),
            (xd, yd, zd),
        ]
        .into_iter()
    }

    fn clamp_to(&mut self, arg: Range<i32>) {
        let Self { x, y, z } = self;

        x.start = max(x.start, arg.start);
        x.end = min(x.end, arg.end);
        y.start = max(y.start, arg.start);
        y.end = min(y.end, arg.end);
        z.start = max(z.start, arg.start);
        z.end = min(z.end, arg.end);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT_0: &str = include_str!("../test-input-0");
    const TEST_INPUT_1: &str = include_str!("../test-input-1");

    #[test]
    fn test_part1_0() {
        assert_eq!(39, n_cubes_on_restricted(TEST_INPUT_0));
    }

    #[test]
    fn test_part1_1() {
        assert_eq!(590784, n_cubes_on_restricted(TEST_INPUT_1));
    }

    #[test]
    fn space_volume() {
        let s = Space::new(10..11, 10..11, 10..11);
        assert_eq!(1, s.volume());

        let s = Space::new(10..13, 10..13, 10..13);
        assert_eq!(27, s.volume());
    }

    #[test]
    fn space_split_at() {
        let s = Space::new(0..4, 0..4, 0..4);
        assert_eq!(64, s.volume());

        let c = s.split_at((0, 0, 0));
        let v = c.map(|s| s.volume());
        assert_eq!([0, 0, 0, 0, 0, 0, 0, 64], v);
        assert_eq!(64usize, v.iter().sum());

        let c = s.split_at((1, 1, 1));
        let v = c.map(|s| s.volume());
        assert_eq!([1, 3, 3, 9, 3, 9, 9, 27], v);
        assert_eq!(64usize, v.iter().sum());

        let c = s.split_at((2, 2, 2));
        let v = c.map(|s| s.volume());
        assert_eq!([8, 8, 8, 8, 8, 8, 8, 8], v);
        assert_eq!(64usize, v.iter().sum());

        let c = s.split_at((3, 3, 3));
        let v = c.map(|s| s.volume());
        assert_eq!([27, 9, 9, 3, 9, 3, 3, 1], v);
        assert_eq!(64usize, v.iter().sum());

        let c = s.split_at((4, 4, 4));
        let v = c.map(|s| s.volume());
        assert_eq!([64, 0, 0, 0, 0, 0, 0, 0], v);
        assert_eq!(64usize, v.iter().sum());
    }

    #[test]
    fn space_split_at_miss() {
        let s = Space::new(0..1, 0..1, 0..1);
        assert_eq!(1, s.volume());

        let c = s.split_at((10, 10, 10));
        let v = c.map(|s| s.volume());
        assert_eq!([1, 0, 0, 0, 0, 0, 0, 0], v);
        assert_eq!(1usize, v.iter().sum());
    }

    #[test]
    fn space_completely_contains() {
        let s = Space::new(1..2, 1..2, 1..2);
        assert!(s.completely_contains(&s));
    }

    #[test]
    fn space_subtract() {
        let a = Space::new(0..3, 0..3, 0..3);
        let b = Space::new(1..2, 1..2, 1..2);
        assert_eq!(27, a.volume());
        assert_eq!(1, b.volume());

        let results = a.subtract(&b);
        assert_eq!(26, results.len());
        assert_eq!(26usize, results.iter().map(Space::volume).sum());
    }
}
